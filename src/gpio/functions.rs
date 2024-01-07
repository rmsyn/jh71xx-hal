//! GPIO function multiplexer (FMUX)

pub trait Function {
    const GROUP: GpioGroup;
    const INDEX: u8;
}

/// Represents the GPIO signal group for configurable functions.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GpioGroup {
    #[default]
    Gpo,
    Gpen,
    Gpi,
    AonGpo,
    AonGpen,
    AonGpi,
}

impl GpioGroup {
    /// Creates a new [GpioGroup].
    pub const fn new() -> Self {
        Self::Gpo
    }
}

/// Configurable GPO function indices.
///
/// GPO (DOUT) function signals can be configured for GPIO pins 0-63.
pub struct GpoFunction;

impl GpoFunction {
    pub const U0_WAVE511_O_UART_TXSOUT: u8 = 2;
    pub const U0_CAN_CTRL_STBY: u8 = 3;
    pub const U0_CAN_CTRL_TST_NEXT_BIT: u8 = 4;
    pub const U0_CAN_CTRL_TST_SAMPLE_POINT: u8 = 5;
    pub const U0_CAN_CTRL_TXD: u8 = 6;
    pub const U0_CDN_USB_DRIVE_VBUS_IO: u8 = 7;
    pub const U0_CDNS_QSPI_CSN1: u8 = 8;
    pub const U0_CDNS_SPDIF_SPDIFO: u8 = 9;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_OUT: u8 = 10;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_OUT: u8 = 11;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_OUT: u8 = 12;
    pub const U0_DSKIT_WDT_WDOGRES: u8 = 13;
    pub const U0_DW_I2C_IC_CLK_OUT_A: u8 = 14;
    pub const U0_DW_I2C_IC_DATA_OUT_A: u8 = 15;
    pub const U0_DW_SDIO_BACK_END_POWER: u8 = 16;
    pub const U0_DW_SDIO_CARD_POWER_EN: u8 = 17;
    pub const U0_DW_SDIO_CCMD_OD_PULLUP_EN_N: u8 = 18;
    pub const U0_DW_SDIO_RST_N: u8 = 19;
    pub const U0_DW_UART_SOUT: u8 = 20;
    pub const U0_HIFI4_JTDO: u8 = 21;
    pub const U0_JTAG_CERTIFICATION_TDO: u8 = 22;
    pub const U0_PDM_4MIC_DMIC_MCLK: u8 = 23;
    pub const U0_PWM_8CH_PTC_PWM_0: u8 = 24;
    pub const U0_PWM_8CH_PTC_PWM_1: u8 = 25;
    pub const U0_PWM_8CH_PTC_PWM_2: u8 = 26;
    pub const U0_PWM_8CH_PTC_PWM_3: u8 = 27;
    pub const U0_PWMDAC_PWMDAC_LEFT_OUTPUT: u8 = 28;
    pub const U0_PWMDAC_PWMDAC_RIGHT_OUTPUT: u8 = 29;
    pub const U0_SSP_SPI_SSPCLKOUT: u8 = 30;
    pub const U0_SSP_SPI_SSPFSSOUT: u8 = 31;
    pub const U0_SSP_SPI_SSPTXD: u8 = 32;
    pub const U0_SYS_CRG_CLK_GMAC_PHY: u8 = 33;
    pub const U0_SYS_CRG_I2SRX_BCLK_MST: u8 = 34;
    pub const U0_SYS_CRG_I2SRX_LRCK_MST: u8 = 35;
    pub const U0_SYS_CRG_I2STX_BCLK_MST: u8 = 36;
    pub const U0_SYS_CRG_I2STX_LRCK_MST: u8 = 37;
    pub const U0_SYS_CRG_MCLK_OUT: u8 = 38;
    pub const U0_SYS_CRG_TDM_CLK_MST: u8 = 39;
    pub const U0_TDM16SLOT_PCM_SYNCOUT: u8 = 40;
    pub const U0_TDM16SLOT_PCM_TXD: u8 = 41;
    pub const U0_U7MC_SFT7110_TRACE_COM_PIB_TDATA_0: u8 = 42;
    pub const U0_U7MC_SFT7110_TRACE_COM_PIB_TDATA_1: u8 = 43;
    pub const U0_U7MC_SFT7110_TRACE_COM_PIB_TDATA_2: u8 = 44;
    pub const U0_U7MC_SFT7110_TRACE_COM_PIB_TDATA_3: u8 = 45;
    pub const U0_U7MC_SFT7110_TRACE_COM_PIB_TREF: u8 = 46;
    pub const U1_CAN_CTRL_STBY: u8 = 47;
    pub const U1_CAN_CTRL_TST_NEXT_BIT: u8 = 48;
    pub const U1_CAN_CTRL_TST_SAMPLE_POINT: u8 = 49;
    pub const U1_CAN_CTRL_TXD: u8 = 50;
    pub const U1_DW_I2C_IC_CLK_OUT_A: u8 = 51;
    pub const U1_DW_I2C_IC_DATA_OUT_A: u8 = 52;
    pub const U1_DW_SDIO_BACK_END_POWER: u8 = 53;
    pub const U1_DW_SDIO_CARD_POWER_EN: u8 = 54;
    pub const U1_DW_SDIO_CCLK_OUT: u8 = 55;
    pub const U1_DW_SDIO_CCMD_OD_PULLUP_EN_N: u8 = 56;
    pub const U1_DW_SDIO_CCMD_OUT: u8 = 57;
    pub const U1_DW_SDIO_CDATA_OUT_0: u8 = 58;
    pub const U1_DW_SDIO_CDATA_OUT_1: u8 = 59;
    pub const U1_DW_SDIO_CDATA_OUT_2: u8 = 60;
    pub const U1_DW_SDIO_CDATA_OUT_3: u8 = 61;
    pub const U1_DW_SDIO_CDATA_OUT_4: u8 = 62;
    pub const U1_DW_SDIO_CDATA_OUT_5: u8 = 63;
    pub const U1_DW_SDIO_CDATA_OUT_6: u8 = 64;
    pub const U1_DW_SDIO_CDATA_OUT_7: u8 = 65;
    pub const U1_DW_SDIO_RST_N: u8 = 66;
    pub const U1_DW_UART_RTS_N: u8 = 67;
    pub const U1_DW_UART_SOUT: u8 = 68;
    pub const U1_I2STX_4CH_SDO0: u8 = 69;
    pub const U1_I2STX_4CH_SDO1: u8 = 70;
    pub const U1_I2STX_4CH_SDO2: u8 = 71;
    pub const U1_I2STX_4CH_SDO3: u8 = 72;
    pub const U1_SSP_SPI_SSPCLKOUT: u8 = 73;
    pub const U1_SSP_SPI_SSPFSSOUT: u8 = 74;
    pub const U1_SSP_SPI_SSPTXD: u8 = 75;
    pub const U2_DW_I2C_IC_CLK_OUT_A: u8 = 76;
    pub const U2_DW_I2C_IC_DATA_OUT_A: u8 = 77;
    pub const U2_DW_UART_RTS_N: u8 = 78;
    pub const U2_DW_UART_SOUT: u8 = 79;
    pub const U2_SSP_SPI_SSPCLKOUT: u8 = 80;
    pub const U2_SSP_SPI_SSPFSSOUT: u8 = 81;
    pub const U2_SSP_SPI_SSPTXD: u8 = 82;
    pub const U3_DW_I2C_IC_CLK_OUT_A: u8 = 83;
    pub const U3_DW_I2C_IC_DATA_OUT_A: u8 = 84;
    pub const U3_DW_UART_SOUT: u8 = 85;
    pub const U3_SSP_SPI_SSPCLKOUT: u8 = 86;
    pub const U3_SSP_SPI_SSPFSSOUT: u8 = 87;
    pub const U3_SSP_SPI_SSPTXD: u8 = 88;
    pub const U4_DW_I2C_IC_CLK_OUT_A: u8 = 89;
    pub const U4_DW_I2C_IC_DATA_OUT_A: u8 = 90;
    pub const U4_DW_UART_RTS_N: u8 = 91;
    pub const U4_DW_UART_SOUT: u8 = 92;
    pub const U4_SSP_SPI_SSPCLKOUT: u8 = 93;
    pub const U4_SSP_SPI_SSPFSSOUT: u8 = 94;
    pub const U4_SSP_SPI_SSPTXD: u8 = 95;
    pub const U5_DW_I2C_IC_CLK_OUT_A: u8 = 96;
    pub const U5_DW_I2C_IC_DATA_OUT_A: u8 = 97;
    pub const U5_DW_UART_RTS_N: u8 = 98;
    pub const U5_DW_UART_SOUT: u8 = 99;
    pub const U5_SSP_SPI_SSPCLKOUT: u8 = 100;
    pub const U5_SSP_SPI_SSPFSSOUT: u8 = 101;
    pub const U5_SSP_SPI_SSPTXD: u8 = 102;
    pub const U6_DW_I2C_IC_CLK_OUT_A: u8 = 103;
    pub const U6_DW_I2C_IC_DATA_OUT_A: u8 = 104;
    pub const U6_SSP_SPI_SSPCLKOUT: u8 = 105;
    pub const U6_SSP_SPI_SSPFSSOUT: u8 = 106;
    pub const U6_SSP_SPI_SSPTXD: u8 = 107;
}

/// Configurable GPEN function indices.
///
/// GPEN (DOEN) function signals can be configured for GPIO pins 0-63.
pub struct GpenFunction;

impl GpenFunction {
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_OEN: u8 = 2;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_OEN: u8 = 3;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_OEN: u8 = 4;
    pub const U0_DW_I2C_IC_CLK_OE: u8 = 5;
    pub const U0_DW_I2C_IC_DATA_OE: u8 = 6;
    pub const U0_HIFI4_JTDOEN: u8 = 7;
    pub const U0_JTAG_CERTIFICATION_TDO_OE: u8 = 8;
    pub const U0_PWM_8CH_PTC_OE_N_0: u8 = 9;
    pub const U0_PWM_8CH_PTC_OE_N_1: u8 = 10;
    pub const U0_PWM_8CH_PTC_OE_N_2: u8 = 11;
    pub const U0_PWM_8CH_PTC_OE_N_3: u8 = 12;
    pub const U0_SSP_SPI_NSSPCTLOE: u8 = 13;
    pub const U0_SSP_SPI_NSSPOE: u8 = 14;
    pub const U0_TDM16SLOT_NPCM_SYNCOE: u8 = 15;
    pub const U0_TDM16SLOT_NPCM_TXDOE: u8 = 16;
    pub const U1_DW_I2C_IC_CLK_OE: u8 = 17;
    pub const U1_DW_I2C_IC_DATA_OE: u8 = 18;
    pub const U1_DW_SDIO_CCMD_OUT_EN: u8 = 19;
    pub const U1_DW_SDIO_CDATA_OUT_EN_0: u8 = 20;
    pub const U1_DW_SDIO_CDATA_OUT_EN_1: u8 = 21;
    pub const U1_DW_SDIO_CDATA_OUT_EN_2: u8 = 22;
    pub const U1_DW_SDIO_CDATA_OUT_EN_3: u8 = 23;
    pub const U1_DW_SDIO_CDATA_OUT_EN_4: u8 = 24;
    pub const U1_DW_SDIO_CDATA_OUT_EN_5: u8 = 25;
    pub const U1_DW_SDIO_CDATA_OUT_EN_6: u8 = 26;
    pub const U1_DW_SDIO_CDATA_OUT_EN_7: u8 = 27;
    pub const U1_SSP_SPI_NSSPCTLOE: u8 = 28;
    pub const U1_SSP_SPI_NSSPOE: u8 = 29;
    pub const U2_DW_I2C_IC_CLK_OE: u8 = 30;
    pub const U2_DW_I2C_IC_DATA_OE: u8 = 31;
    pub const U2_SSP_SPI_NSSPCTLOE: u8 = 32;
    pub const U2_SSP_SPI_NSSPOE: u8 = 33;
    pub const U3_DW_I2C_IC_CLK_OE: u8 = 34;
    pub const U3_DW_I2C_IC_DATA_OE: u8 = 35;
    pub const U3_SSP_SPI_NSSPCTLOE: u8 = 36;
    pub const U3_SSP_SPI_NSSPOE: u8 = 37;
    pub const U4_DW_I2C_IC_CLK_OE: u8 = 38;
    pub const U4_DW_I2C_IC_DATA_OE: u8 = 39;
    pub const U4_SSP_SPI_NSSPCTLOE: u8 = 40;
    pub const U4_SSP_SPI_NSSPOE: u8 = 41;
    pub const U5_DW_I2C_IC_CLK_OE: u8 = 42;
    pub const U5_DW_I2C_IC_DATA_OE: u8 = 43;
    pub const U5_SSP_SPI_NSSPCTLOE: u8 = 44;
    pub const U5_SSP_SPI_NSSPOE: u8 = 45;
    pub const U6_DW_I2C_IC_CLK_OE: u8 = 46;
    pub const U6_DW_I2C_IC_DATA_OE: u8 = 47;
    pub const U6_SSP_SPI_NSSPCTLOE: u8 = 48;
    pub const U6_SSP_SPI_NSSPOE: u8 = 49;
}

/// Configurable GPEN function indices.
///
/// GPI function signals can be configured for GPIO pins 2-63 (GPIO0-GPIO1 are reserved).
pub struct GpiFunction;

impl GpiFunction {
    pub const U0_WAVE511_I_UART_RXSIN: u8 = 0;
    pub const U0_CAN_CTRL_RXD: u8 = 1;
    pub const U0_CDN_USB_OVERCURRENT_N_IO: u8 = 2;
    pub const U0_CDNS_SPDIF_SPDIFI: u8 = 3;
    pub const U0_CLKRST_SRC_BYPASS_JTAG_TRSTN: u8 = 4;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_CEC_SDA_IN: u8 = 5;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SCL_IN: u8 = 6;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_DDC_SDA_IN: u8 = 7;
    pub const U0_DOM_VOUT_TOP_U0_HDMI_TX_PIN_HPD: u8 = 8;
    pub const U0_DW_I2C_IC_CLK_IN_A: u8 = 9;
    pub const U0_DW_I2C_IC_DATA_IN_A: u8 = 10;
    pub const U0_DW_SDIO_CARD_DETECT_N: u8 = 11;
    pub const U0_DW_SDIO_CARD_INT_N: u8 = 12;
    pub const U0_DW_SDIO_CARD_WRITE_PRT: u8 = 13;
    pub const U0_DW_UART_SIN: u8 = 14;
    pub const U0_HIFI4_JTCK: u8 = 15;
    pub const U0_HIFI4_JTDI: u8 = 16;
    pub const U0_HIFI4_JTMS: u8 = 17;
    pub const U0_HIFI4_JTRSTN: u8 = 18;
    pub const U0_JTAG_CERTIFICATION_TDI: u8 = 19;
    pub const U0_JTAG_CERTIFICATION_TMS: u8 = 20;
    pub const U0_PDM_4MIC_DMIC0_DIN: u8 = 21;
    pub const U0_PDM_4MIC_DMIC1_DIN: u8 = 22;
    pub const U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN0: u8 = 23;
    pub const U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN1: u8 = 24;
    pub const U0_SAIF_AUDIO_SDIN_MUX_I2SRX_EXT_SDIN2: u8 = 25;
    pub const U0_SSP_SPI_SSPCLKIN: u8 = 26;
    pub const U0_SSP_SPI_SSPFSSIN: u8 = 27;
    pub const U0_SSP_SPI_SSPRXD: u8 = 28;
    pub const U0_SYS_CRG_CLK_JTAG_TCK: u8 = 29;
    pub const U0_SYS_CRG_EXT_MCLK: u8 = 30;
    pub const U0_SYS_CRG_I2SRX_BCLK_SLV: u8 = 31;
    pub const U0_SYS_CRG_I2SRX_LRCK_SLV: u8 = 32;
    pub const U0_SYS_CRG_I2STX_BCLK_SLV: u8 = 33;
    pub const U0_SYS_CRG_I2STX_LRCK_SLV: u8 = 34;
    pub const U0_SYS_CRG_TDM_CLK_SLV: u8 = 35;
    pub const U0_TDM16SLOT_PCM_RXD: u8 = 36;
    pub const U0_TDM16SLOT_PCM_SYNCIN: u8 = 37;
    pub const U1_CAN_CTRL_RXD: u8 = 38;
    pub const U1_DW_I2C_IC_CLK_IN_A: u8 = 39;
    pub const U1_DW_I2C_IC_DATA_IN_A: u8 = 40;
    pub const U1_DW_SDIO_CARD_DETECT_N: u8 = 41;
    pub const U1_DW_SDIO_CARD_INT_N: u8 = 42;
    pub const U1_DW_SDIO_CARD_WRITE_PRT: u8 = 43;
    pub const U1_DW_SDIO_CCMD_IN: u8 = 44;
    pub const U1_DW_SDIO_CDATA_IN_0: u8 = 45;
    pub const U1_DW_SDIO_CDATA_IN_1: u8 = 46;
    pub const U1_DW_SDIO_CDATA_IN_2: u8 = 47;
    pub const U1_DW_SDIO_CDATA_IN_3: u8 = 48;
    pub const U1_DW_SDIO_CDATA_IN_4: u8 = 49;
    pub const U1_DW_SDIO_CDATA_IN_5: u8 = 50;
    pub const U1_DW_SDIO_CDATA_IN_6: u8 = 51;
    pub const U1_DW_SDIO_CDATA_IN_7: u8 = 52;
    pub const U1_DW_SDIO_DATA_STROBE: u8 = 53;
    pub const U1_DW_UART_CTS_N: u8 = 54;
    pub const U1_DW_UART_SIN: u8 = 55;
    pub const U1_SSP_SPI_SSPCLKIN: u8 = 56;
    pub const U1_SSP_SPI_SSPFSSIN: u8 = 57;
    pub const U1_SSP_SPI_SSPRXD: u8 = 58;
    pub const U2_DW_I2C_IC_CLK_IN_A: u8 = 59;
    pub const U2_DW_I2C_IC_DATA_IN_A: u8 = 60;
    pub const U2_DW_UART_CTS_N: u8 = 61;
    pub const U2_DW_UART_SIN: u8 = 62;
    pub const U2_SSP_SPI_SSPCLKIN: u8 = 63;
    pub const U2_SSP_SPI_SSPFSSIN: u8 = 64;
    pub const U2_SSP_SPI_SSPRXD: u8 = 65;
    pub const U3_DW_I2C_IC_CLK_IN_A: u8 = 66;
    pub const U3_DW_I2C_IC_DATA_IN_A: u8 = 67;
    pub const U3_DW_UART_SIN: u8 = 68;
    pub const U3_SSP_SPI_SSPCLKIN: u8 = 69;
    pub const U3_SSP_SPI_SSPFSSIN: u8 = 70;
    pub const U3_SSP_SPI_SSPRXD: u8 = 71;
    pub const U4_DW_I2C_IC_CLK_IN_A: u8 = 72;
    pub const U4_DW_I2C_IC_DATA_IN_A: u8 = 73;
    pub const U4_DW_UART_CTS_N: u8 = 74;
    pub const U4_DW_UART_SIN: u8 = 75;
    pub const U4_SSP_SPI_SSPCLKIN: u8 = 76;
    pub const U4_SSP_SPI_SSPFSSIN: u8 = 77;
    pub const U4_SSP_SPI_SSPRXD: u8 = 78;
    pub const U5_DW_I2C_IC_CLK_IN_A: u8 = 79;
    pub const U5_DW_I2C_IC_DATA_IN_A: u8 = 80;
    pub const U5_DW_UART_CTS_N: u8 = 81;
    pub const U5_DW_UART_SIN: u8 = 82;
    pub const U5_SSP_SPI_SSPCLKIN: u8 = 83;
    pub const U5_SSP_SPI_SSPFSSIN: u8 = 84;
    pub const U5_SSP_SPI_SSPRXD: u8 = 85;
    pub const U6_DW_I2C_IC_CLK_IN_A: u8 = 86;
    pub const U6_DW_I2C_IC_DATA_IN_A: u8 = 87;
    pub const U6_SSP_SPI_SSPCLKIN: u8 = 88;
    pub const U6_SSP_SPI_SSPFSSIN: u8 = 89;
    pub const U6_SSP_SPI_SSPRXD: u8 = 90;
}

/// Configurable AON GPO function indices.
///
/// AON GPO (Always-on DOUT) function signals can be configured for GPIO pins 0-63.
pub struct AonGpoFunction;

impl AonGpoFunction {
    pub const U0_AON_CRG_CLK_32K_OUT: u8 = 2;
    pub const U0_PWM_8CH_PTC_PWM_4: u8 = 3;
    pub const U0_PWM_8CH_PTC_PWM_5: u8 = 4;
    pub const U0_PWM_8CH_PTC_PWM_6: u8 = 5;
    pub const U0_PWM_8CH_PTC_PWM_7: u8 = 6;
    pub const U0_SYS_CRG_CLK_GCLK0: u8 = 7;
    pub const U0_SYS_CRG_CLK_GCLK1: u8 = 8;
    pub const U0_SYS_CRG_CLK_GCLK2: u8 = 9;
}

/// Configurable AON GPEN function indices.
///
/// AON GPEN (Always-on DOEN) function signals can be configured for GPIO pins 0-63.
pub struct AonGpenFunction;

impl AonGpenFunction {
    pub const U0_PWM_8CH_PTC_OE_N_4: u8 = 2;
    pub const U0_PWM_8CH_PTC_OE_N_5: u8 = 3;
    pub const U0_PWM_8CH_PTC_OE_N_6: u8 = 4;
    pub const U0_PWM_8CH_PTC_OE_N_7: u8 = 5;
}

/// Configurable AON GPI function indices.
///
/// AON GPI (Always-on DIN) function signals can be configured for GPIO pins 2-63 (GPIO0-GPIO1 are reserved).
pub struct AonGpiFunction;

impl AonGpiFunction {
    pub const U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_0: u8 = 0;
    pub const U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_1: u8 = 1;
    pub const U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_2: u8 = 2;
    pub const U0_PMU_IO_EVENT_STUB_GPIO_WAKEUP_3: u8 = 3;
}
