#include <stdint.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include "../gen/anx7625.h"
#include "anx7625.h"
#include "anx7625_pub.h"
#include "logging.h"

int32_t anx7625_init(AnxFnPtrs *anx, void *bus, void *delay, void *video_on, void *video_rst, void *otg_on)
{

    int32_t retry_power_on = 3;

    ANXINFO("OTG_ON = 1 -> VBUS OFF");
    anx->set_pin(anx, otg_on, true);

    anx->delay(anx, delay, 1000);
    anx->set_pin(anx, video_on, true);
    anx->delay(anx, delay, 10);
    anx->set_pin(anx, video_rst, true);
    anx->delay(anx, delay, 10);

    anx->set_pin(anx, video_on, false);
    anx->delay(anx, delay, 10);
    anx->set_pin(anx, video_rst, false);
    anx->delay(anx, delay, 100);

    ANXINFO("Powering on anx7625...");
    anx->set_pin(anx, video_on, true);
    anx->delay(anx, delay, 100);
    anx->set_pin(anx, video_rst, true);

    while (--retry_power_on)
    {
        if (anx7625_power_on_init(anx, bus, delay) == 0)
        {
            break;
        }
    }
    if (!retry_power_on)
    {
        ANXERROR("Failed to power on.");
        return -1;
    }
    ANXINFO("Powering on anx7625 successfull.");
    anx->delay(anx, delay, 200); // Wait for anx7625 to be stable

    if (anx7625_is_power_provider(anx, bus))
    {
        ANXINFO("OTG_ON = 0 -> VBUS ON");
        anx->set_pin(anx, otg_on, false);
        anx->delay(anx, delay, 1000); // Wait for powered device to be stable
    }

    return 0;
}

void anx7625_wait_hpd_event(AnxFnPtrs *anx, void *bus, void *delay)
{
    ANXINFO("Waiting for hdmi hot plug event...");
    while (1)
    {
        anx->delay(anx, delay, 10);
        int detected = anx7625_hpd_change_detect(anx, bus);
        if (detected == 1)
            break;
    }
}

int32_t anx7625_dp_get_edid(AnxFnPtrs *anx, void *bus, void *delay, struct edid *out)
{
    int32_t block_num;
    int32_t ret;
    uint8_t edid[FOUR_BLOCK_SIZE];

    block_num = sp_tx_edid_read(anx, bus, delay, edid, FOUR_BLOCK_SIZE);
    if (block_num < 0)
    {
        ANXERROR("Failed to get eDP EDID.");
        return -1;
    }

    ret = decode_edid(edid, (block_num + 1) * ONE_BLOCK_SIZE, out);
    if (ret != EDID_CONFORMANT)
    {
        ANXERROR("Failed to decode EDID.");
        return -1;
    }

    return 0;
}

/////////////////////////////
// END OF PUBLIC FUNCTIONS //
/////////////////////////////

int32_t segments_edid_read(AnxFnPtrs *anx, void *bus, void *delay, uint8_t segment, uint8_t *buf, uint8_t offset)
{
    uint8_t c, cnt = 0;
    int32_t ret;

    /* write address only */
    ret = anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_ADDR_7_0, 0x30);
    ret |= anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_COMMAND, 0x04);
    ret |= anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_CTRL_STATUS, AP_AUX_CTRL_ADDRONLY | AP_AUX_CTRL_OP_EN);

    ret |= wait_aux_op_finish(anx, bus, delay);
    /* write segment address */
    ret |= sp_tx_aux_wr(anx, bus, delay, segment);
    /* data read */
    ret |= anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_ADDR_7_0, 0x50);

    if (ret < 0)
    {
        ANXERROR("IO error: aux initial failed.\n");
        return ret;
    }

    for (cnt = 0; cnt < 3; cnt++)
    {
        sp_tx_aux_wr(anx, bus, delay, offset);
        /* set I2C read com 0x01 mot = 0 and read 16 bytes */
        c = sp_tx_aux_rd(anx, bus, delay, 0xf1);

        if (c == 1)
        {
            ret = sp_tx_rst_aux(anx, bus);
            ANXERROR("segment read failed, reset!\n");
            cnt++;
        }
        else
        {
            ret = anx7625_reg_block_read(bus, RX_P0_ADDR,
                                         AP_AUX_BUFF_START,
                                         MAX_DPCD_BUFFER_SIZE, buf);
            return ret;
        }
    }

    return ret;
}

int32_t edid_read(AnxFnPtrs *anx, void *bus, void *delay, uint8_t offset, uint8_t *pblock_buf)
{
    uint8_t c, cnt = 0;

    c = 0;
    for (cnt = 0; cnt < 3; cnt++)
    {
        sp_tx_aux_wr(anx, bus, delay, offset);
        /* set I2C read com 0x01 mot = 0 and read 16 bytes */
        c = sp_tx_aux_rd(anx, bus, delay, 0xf1);

        if (c == 1)
        {
            sp_tx_rst_aux(anx, bus);
            ANXERROR("edid read failed, reset!\n");
            cnt++;
        }
        else
        {
            anx7625_reg_block_read(bus, RX_P0_ADDR,
                                   AP_AUX_BUFF_START,
                                   MAX_DPCD_BUFFER_SIZE, pblock_buf);
            return 0;
        }
    }

    return 1;
}

int32_t sp_tx_aux_rd(AnxFnPtrs *anx, void *bus, void *delay, uint8_t len_cmd)
{
    int32_t ret;

    ret = anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_COMMAND, len_cmd);
    ret |= anx7625_write_or(anx, bus, RX_P0_ADDR, AP_AUX_CTRL_STATUS, AP_AUX_CTRL_OP_EN);
    return ret | wait_aux_op_finish(anx, bus, delay);
}

int32_t wait_aux_op_finish(AnxFnPtrs *anx, void *bus, void *delay)
{
    uint8_t val;
    int32_t ret = -1;
    int32_t loop;

    for (loop = 0; loop < 150; loop++)
    {
        anx->delay(anx, delay, 2);
        anx7625_reg_read(anx, bus, RX_P0_ADDR, AP_AUX_CTRL_STATUS, &val);
        if (!(val & AP_AUX_CTRL_OP_EN))
        {
            ret = 0;
            break;
        }
    }

    if (ret != 0)
    {
        ANXERROR("Timed out waiting aux operation.\n");
        return ret;
    }

    ret = anx7625_reg_read(anx, bus, RX_P0_ADDR, AP_AUX_CTRL_STATUS, &val);
    if (ret < 0 || val & 0x0F)
    {
        ANXDEBUG("aux status %02x\n", val);
        ret = -1;
    }

    return ret;
}

int32_t sp_tx_aux_wr(AnxFnPtrs *anx, void *bus, void *delay, uint8_t offset)
{
    int32_t ret;

    ret = anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_BUFF_START, offset);
    ret |= anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_COMMAND, 0x04);
    ret |= anx7625_write_or(anx, bus, RX_P0_ADDR, AP_AUX_CTRL_STATUS, AP_AUX_CTRL_OP_EN);
    return ret | wait_aux_op_finish(anx, bus, delay);
}

int32_t sp_tx_get_edid_block(AnxFnPtrs *anx, void *bus, void *delay)
{
    int32_t ret;
    uint8_t val = 0;

    sp_tx_aux_wr(anx, bus, delay, 0x7e);
    sp_tx_aux_rd(anx, bus, delay, 0x01);
    ret = anx7625_reg_read(anx, bus, RX_P0_ADDR, AP_AUX_BUFF_START, &val);

    if (ret < 0)
    {
        ANXERROR("IO error: access AUX BUFF.\n");
        return -1;
    }

    ANXINFO("EDID Block = %d", val + 1);

    if (val > 3)
        val = 1;

    return val;
}

int32_t sp_tx_rst_aux(AnxFnPtrs *anx, void *bus)
{
    int32_t ret;

    ret = anx7625_write_or(anx, bus, TX_P2_ADDR, RST_CTRL2, AUX_RST);
    ret |= anx7625_write_and(anx, bus, TX_P2_ADDR, RST_CTRL2, ~AUX_RST);
    return ret;
}

int32_t sp_tx_edid_read(AnxFnPtrs *anx, void *bus, void *delay, uint8_t *pedid_blocks_buf, uint32_t size)
{
    uint8_t offset, edid_pos;
    int32_t count, blocks_num;
    uint8_t pblock_buf[MAX_DPCD_BUFFER_SIZE];
    uint8_t i;
    uint8_t g_edid_break = 0;
    int32_t ret;

    /* address initial */
    ret = anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_ADDR_7_0, 0x50);
    ret |= anx7625_reg_write(anx, bus, RX_P0_ADDR, AP_AUX_ADDR_15_8, 0);
    ret |= anx7625_write_and(anx, bus, RX_P0_ADDR, AP_AUX_ADDR_19_16, 0xf0);

    if (ret < 0)
    {
        ANXERROR("access aux channel IO error.\n");
        return -1;
    }

    blocks_num = sp_tx_get_edid_block(anx, bus, delay);
    if (blocks_num < 0)
        return blocks_num;

    count = 0;

    do
    {
        switch (count)
        {
        case 0:
        case 1:
            for (i = 0; i < 8; i++)
            {
                offset = (i + count * 8) * MAX_DPCD_BUFFER_SIZE;
                g_edid_break = edid_read(anx, bus, delay, offset, pblock_buf);

                if (g_edid_break == 1)
                    break;

                if (offset <= size - MAX_DPCD_BUFFER_SIZE)
                    memcpy(&pedid_blocks_buf[offset],
                           pblock_buf,
                           MAX_DPCD_BUFFER_SIZE);
            }

            break;
        case 2:
        case 3:
            offset = (count == 2) ? 0x00 : 0x80;

            for (i = 0; i < 8; i++)
            {
                edid_pos = (i + count * 8) *
                           MAX_DPCD_BUFFER_SIZE;

                if (g_edid_break == 1)
                    break;

                segments_edid_read(anx, bus, delay, count / 2, pblock_buf, offset);
                if (edid_pos <= size - MAX_DPCD_BUFFER_SIZE)
                    memcpy(&pedid_blocks_buf[edid_pos],
                           pblock_buf,
                           MAX_DPCD_BUFFER_SIZE);
                offset = offset + 0x10;
            }

            break;
        default:
            die("%s: count should be <= 3", __func__);
            break;
        }

        count++;

    } while (blocks_num >= count);

    /* reset aux channel */
    sp_tx_rst_aux(anx, bus);

    return blocks_num;
}

int32_t anx7625_read_system_status(AnxFnPtrs *anx, void *bus, uint8_t *sys_status)
{
    int32_t ret = 0;
    ret = anx7625_reg_read(anx, bus, RX_P0_ADDR, SYSTEM_STSTUS, sys_status); // 0x7e, 0x45
    if (ret < 0)
    {
        ANXERROR("Failed %s", __func__);
        return ret;
    }
    if (*sys_status & (1 << 2))
        ANXDEBUG("anx: - VCONN status ON\n");
    if (!(*sys_status & (1 << 2)))
        ANXDEBUG("anx: - VCONN status OFF\n");

    if (*sys_status & (1 << 3))
        ANXDEBUG("anx: - VBUS power provider\n");
    if (!(*sys_status & (1 << 3)))
        ANXDEBUG("anx: - VBUS power consumer\n");

    if (*sys_status & (1 << 5))
        ANXDEBUG("anx: - Data Role: DFP\n");
    if (!(*sys_status & (1 << 5)))
        ANXDEBUG("anx: - Data Role: UFP\n");

    if (*sys_status & (1 << 7))
        ANXDEBUG("anx: - DP HPD high\n");
    if (!(*sys_status & (1 << 7)))
        ANXDEBUG("anx: - DP HPD low\n");
    return ret;
}

bool anx7625_is_power_provider(AnxFnPtrs *anx, void *bus)
{
    int32_t ret = 0;
    uint8_t sys_status = 0;
    anx7625_read_system_status(anx, bus, &sys_status);
    if (ret < 0)
    {
        ANXERROR("Failed %s", __func__);
        return false; // Conservative
    }
    else
    {
        if (sys_status & (1 << 3))
            return true;
        else
            return false;
    }
}

int32_t anx7625_hpd_change_detect(AnxFnPtrs *anx, void *bus)
{
    int32_t ret;
    uint8_t status;

    ret = anx7625_reg_read(anx, bus, RX_P0_ADDR, SYSTEM_STSTUS, &status);
    if (ret < 0)
    {
        ANXERROR("IO error: Failed to read HPD_STATUS register.");
        return ret;
    }

    if (status & HPD_STATUS)
    {
        ANXINFO("HPD event received 0x7e:0x45=%#x", status);
        anx7625_start_dp_work(anx, bus);
        return 1;
    }
    return 0;
}

void anx7625_start_dp_work(AnxFnPtrs *anx, void *bus)
{
    int32_t ret;
    uint8_t val;

    /* not support HDCP */
    ret = anx7625_write_and(anx, bus, RX_P1_ADDR, 0xee, 0x9f);

    /* try auth flag */
    ret |= anx7625_write_or(anx, bus, RX_P1_ADDR, 0xec, 0x10);
    /* interrupt for DRM */
    ret |= anx7625_write_or(anx, bus, RX_P1_ADDR, 0xff, 0x01);
    if (ret < 0)
        return;

    ret = anx7625_reg_read(anx, bus, RX_P1_ADDR, 0x86, &val);
    if (ret < 0)
        return;

    ANXINFO("Secure OCM version=%02x", val);
}

int32_t anx7625_write_or(AnxFnPtrs *anx, void *bus, uint8_t saddr, uint8_t offset, uint8_t mask)
{
    uint8_t val;
    int32_t ret;

    ret = anx7625_reg_read(anx, bus, saddr, offset, &val);
    if (ret < 0)
        return ret;

    return anx7625_reg_write(anx, bus, saddr, offset, val | mask);
}

int32_t anx7625_write_and(AnxFnPtrs *anx, void *bus, uint8_t saddr, uint8_t offset, uint8_t mask)
{
    int32_t ret;
    uint8_t val;

    ret = anx7625_reg_read(anx, bus, saddr, offset, &val);
    if (ret < 0)
        return ret;

    return anx7625_reg_write(anx, bus, saddr, offset, val & mask);
}

#define FLASH_LOAD_STA 0x05
#define FLASH_LOAD_STA_CHK (1 << 7)

int32_t anx7625_power_on_init(AnxFnPtrs *anx, void *bus, void *delay)
{
    int32_t i, ret;
    uint8_t val, version, revision;

    anx7625_reg_write(anx, bus, RX_P0_ADDR, XTAL_FRQ_SEL, XTAL_FRQ_27M);

    for (i = 0; i < OCM_LOADING_TIME; i++)
    {
        /* check interface */
        ret = anx7625_reg_read(anx, bus, RX_P0_ADDR, FLASH_LOAD_STA, &val);
        if (ret < 0)
        {
            ANXERROR("Failed to load flash");
            return ret;
        }

        if ((val & FLASH_LOAD_STA_CHK) != FLASH_LOAD_STA_CHK)
        {
            anx->delay(anx, delay, 1);
            continue;
        }
        ANXINFO("Init interface.");

        //anx7625_disable_pd_protocol(bus);
        anx7625_reg_read(anx, bus, RX_P0_ADDR, OCM_FW_VERSION, &version);
        anx7625_reg_read(anx, bus, RX_P0_ADDR, OCM_FW_REVERSION, &revision);

        if (version == 0 && revision == 0)
        {
            continue;
        }

        ANXINFO("Firmware: ver 0x%x, rev 0x%x.", version, revision);
        return 0;
    }
    return -1;
}

int32_t i2c_access_workaround(AnxFnPtrs *anx, void *bus, uint8_t saddr)
{
    uint8_t offset;
    static uint8_t saddr_backup = 0;
    int32_t ret = 0;

    if (saddr == saddr_backup)
        return ret;

    saddr_backup = saddr;

    switch (saddr)
    {
    case TCPC_INTERFACE_ADDR:
        offset = RSVD_00_ADDR;
        break;
    case TX_P0_ADDR:
        offset = RSVD_D1_ADDR;
        break;
    case TX_P1_ADDR:
        offset = RSVD_60_ADDR;
        break;
    case RX_P0_ADDR:
        offset = RSVD_39_ADDR;
        break;
    case RX_P1_ADDR:
        offset = RSVD_7F_ADDR;
        break;
    default:
        offset = RSVD_00_ADDR;
        break;
    }

    ret = anx->i2c_writeb(anx, bus, saddr, offset, 0x00);
    if (ret < 0)
        ANXERROR("Failed to access %#x:%#x", saddr, offset);
    return ret;
}

int32_t anx7625_reg_read(AnxFnPtrs *anx, void *bus, uint8_t saddr, uint8_t offset, uint8_t *val)
{
    int32_t ret;

    i2c_access_workaround(anx, bus, saddr);
    ret = anx->i2c_readb(anx, bus, saddr, offset, val);
    if (ret < 0)
    {
        ANXERROR("Failed to read i2c reg=%#x:%#x", saddr, offset);
        return ret;
    }
    return *val;
}

int32_t anx7625_reg_write(AnxFnPtrs *anx, void *bus, uint8_t saddr, uint8_t reg_addr, uint8_t reg_val)
{
    int32_t ret;

    i2c_access_workaround(anx, bus, saddr);
    ret = anx->i2c_writeb(anx, bus, saddr, reg_addr, reg_val);
    if (ret < 0)
        ANXERROR("Failed to write i2c id=%#x:%#x", saddr, reg_addr);

    return ret;
}
