use crate::util::RegexMap;

pub static PERIMAP: RegexMap<(&str, &str, &str)> = RegexMap::new(&[
    (".*:USART:sci2_v1_1", ("usart", "v1", "USART")),
    (".*:USART:sci2_v1_2_F1", ("usart", "v1", "USART")),
    (".*:USART:sci2_v1_2", ("usart", "v2", "USART")),
    (".*:USART:sci2_v2_0", ("usart", "v3", "USART")),
    (".*:USART:sci2_v2_1", ("usart", "v3", "USART")),
    (".*:USART:sci2_v2_2", ("usart", "v3", "USART")),
    (".*:USART:sci3_v1_0", ("usart", "v3", "USART")),
    (".*:USART:sci3_v1_1", ("usart", "v3", "USART")),
    (".*:USART:sci3_v1_2", ("usart", "v4", "USART")),
    (".*:USART:sci3_v2_0", ("usart", "v4", "USART")),
    (".*:USART:sci3_v2_1", ("usart", "v4", "USART")),
    (".*:UART:sci2_v1_1", ("usart", "v1", "USART")),
    (".*:UART:sci2_v1_2_F4", ("usart", "v2", "USART")),
    (".*:UART:sci2_v2_1", ("usart", "v3", "USART")),
    (".*:UART:sci2_v3_0", ("usart", "v4", "USART")),
    (".*:UART:sci2_v3_1", ("usart", "v4", "USART")),
    (".*:LPUART:sci3_v1_1", ("usart", "v3", "LPUART")),
    (".*:LPUART:sci3_v1_2", ("usart", "v4", "LPUART")),
    (".*:LPUART:sci3_v1_3", ("usart", "v4", "LPUART")),
    (".*:LPUART:sci3_v1_4", ("usart", "v4", "LPUART")),
    ("STM32[HU]5.*:RNG:.*", ("rng", "v3", "RNG")),
    ("STM32U0.*:RNG:.*", ("rng", "v3", "RNG")),
    ("STM32L5.*:RNG:.*", ("rng", "v2", "RNG")),
    ("STM32L4[PQ]5.*:RNG:.*", ("rng", "v2", "RNG")),
    ("STM32WL.*:RNG:.*", ("rng", "v2", "RNG")),
    ("STM32F2.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32F4.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32F7.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32L0.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32L4.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32H7.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32G0.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32G4.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32WB.*:RNG:.*", ("rng", "v1", "RNG")),
    ("STM32F7.*:AES:.*", ("aes", "f7", "AES")),
    ("STM32F4.*:AES:.*", ("aes", "v1", "AES")),
    ("STM32G0.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32U0.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32G4.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32L0.*:AES:.*", ("aes", "v1", "AES")),
    ("STM32L1.*:AES:.*", ("aes", "v1", "AES")),
    ("STM32L4.*:AES:.*", ("aes", "v1", "AES")),
    ("STM32L5.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32WL5.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32WLE.*:AES:.*", ("aes", "v2", "AES")),
    ("STM32U5.*:AES:.*", ("aes", "v3a", "AES")),
    ("STM32(H5|WBA).*:AES:.*", ("aes", "v3b", "AES")),
    ("STM32(H5|WBA).*:SAES:.*", ("saes", "v1a", "SAES")),
    ("STM32U5.*:SAES:.*", ("saes", "v1b", "SAES")),
    ("STM32L0.*:SPI:.*", ("spi", "v1", "SPI")),
    (".*:SPI:spi2_v1_4", ("spi", "f1", "SPI")),
    (".*:SPI:spi2s1_v2_1", ("spi", "v1", "SPI")),
    (".*:SPI:spi2s1_v2_2", ("spi", "v1", "SPI")),
    (".*:SPI:spi2s1_v2_3", ("spi", "v1", "SPI")),
    (".*:SPI:spi2s1_v2_4", ("spi", "v1", "SPI")),
    (".*:SPI:spi2s1_v3_0", ("spi", "v2", "SPI")),
    (".*:SPI:spi2s1_v3_2", ("spi", "v2", "SPI")),
    (".*:SPI:spi2s1_v3_3", ("spi", "v2", "SPI")),
    (".*:SPI:spi2s1_v3_5", ("spi", "v2", "SPI")),
    (".*:SUBGHZSPI:.*", ("spi", "v2", "SPI")),
    (".*:SPI:spi2s1_v3_1", ("spi", "v2", "SPI")),
    (".*:SPI:spi2s2_v1_1", ("spi", "v3", "SPI")),
    (".*:SPI:spi2s2_v1_0", ("spi", "v3", "SPI")),
    (".*:SPI:spi2s3_v2_1", ("spi", "v4", "SPI")),
    (".*:SPI:spi2s3_v1_1", ("spi", "v5", "SPI")),
    (".*:SPI:spi2s3_v2_0", ("spi", "v5", "SPI")),
    (".*:FMAC:matrix1_v1_0", ("fmac", "v1", "FMAC")),
    (".*:I2C:i2c1_v1_5", ("i2c", "v1", "I2C")),
    (".*:I2C:i2c2_v1_1", ("i2c", "v2", "I2C")),
    (".*:I2C:F0-i2c2_v1_1", ("i2c", "v2", "I2C")),
    (".*:I2C:i2c2_v1_1F7", ("i2c", "v2", "I2C")),
    (".*:I2C:i2c2_v1_1U5", ("i2c", "v2", "I2C")),
    (".*:I2C:i2c1_v1_0H7RS", ("i2c", "v3", "I2C")),
    (".*:FMPI2C:i2c2_v1_1", ("fmpi2c", "v2", "FMPI2C")),
    ("STM32F10[1357].*:DAC:dacif_v1_1F1", ("dac", "v1", "DAC")), // Original F1 are v1
    (".*:DAC:dacif_v1_1F1", ("dac", "v2", "DAC")),
    (".*:DAC:F0dacif_v1_1", ("dac", "v2", "DAC")),
    (".*:DAC:F3_dacif_v1_1", ("dac", "v2", "DAC")),
    (".*:DAC:dacif_v1_1", ("dac", "v2", "DAC")),
    (".*:DAC:dacif_v1_2", ("dac", "v2", "DAC")),
    ("STM32L4[1-9A].*:DAC:dacif_v2_0", ("dac", "v3", "DAC")), // L4 non-plus are v3
    (".*:DAC:dacif_v2_0", ("dac", "v5", "DAC")),
    (".*:DAC:dacif_v2_0_U5", ("dac", "v6", "DAC")),
    (".*:DAC:dacif_v2_0_U0", ("dac", "v4", "DAC")),
    (".*:DAC:dacif_v3_0", ("dac", "v4", "DAC")),
    (".*:DAC:WL_dacif_v3_0", ("dac", "v4", "DAC")),
    (".*:DAC:G4_dacif_v4_0", ("dac", "v7", "DAC")),
    (".*:DAC:dacif_v5_0", ("dac", "v6", "DAC")),
    (".*:ADC:aditf_v2_5F1", ("adc", "f1", "ADC")),
    (".*:ADC:aditf5_v1_1", ("adc", "f3", "ADC")),
    (".*:ADC:aditf_v2_5", ("adc", "f3_v2", "ADC")),
    (".*:ADC:aditf3_v1_1", ("adc", "f3_v1_1", "ADC")),
    (".*:ADC:aditf4_v1_1", ("adc", "v1", "ADC")),
    (".*:ADC:aditf4_v2_0", ("adc", "l0", "ADC")),
    (".*:ADC:aditf2_v1_1", ("adc", "v2", "ADC")),
    (".*:ADC:aditf5_v2_0", ("adc", "v3", "ADC")),
    (".*:ADC:aditf5_v2_2", ("adc", "v3", "ADC")),
    (".*:ADC:aditf5_v3_0", ("adc", "v4", "ADC")),
    (".*:ADC:aditf5_v3_0_H5", ("adc", "h5", "ADC")),
    (".*:ADC:aditf512_v3_0_H5", ("adc", "h5", "ADC")),
    (".*:ADC:aditf5_v3_1", ("adc", "v4", "ADC")),
    (".*:ADC:aditf5_40v2_U5", ("adc", "u5", "ADC")),
    (".*:ADC:aditf5_40v1_U5", ("adc", "u5", "ADC")),
    (".*:ADC:aditf4_v4_U5", ("adc", "u5", "ADC4")),
    ("STM32WL5.*:ADC:.*", ("adc", "g0", "ADC")),
    ("STM32WLE.*:ADC:.*", ("adc", "g0", "ADC")),
    ("STM32G0.*:ADC:.*", ("adc", "g0", "ADC")),
    ("STM32U0.*:ADC:.*", ("adc", "u0", "ADC")),
    ("STM32G4.*:ADC:.*", ("adc", "g4", "ADC")),
    ("STM32G0.*:ADC\\d*_COMMON:.*", ("adccommon", "v3", "ADC_COMMON")),
    ("STM32U0.*:ADC\\d*_COMMON:.*", ("adccommon", "v3", "ADC_COMMON")),
    ("STM32G4.*:ADC\\d*_COMMON:.*", ("adccommon", "v4", "ADC_COMMON")),
    ("STM32U5.*:ADC\\d*_COMMON:.*", ("adccommon", "u5", "ADC_COMMON")),
    (
        "STM32(L[45]|W[BL]).*:ADC\\d*_COMMON:.*",
        ("adccommon", "v3", "ADC_COMMON"),
    ),
    ("STM32F3.*:ADC\\d*_COMMON:.*", ("adccommon", "f3", "ADC_COMMON")),
    ("STM32F[247].*:ADC\\d*_COMMON:.*", ("adccommon", "v2", "ADC_COMMON")),
    ("STM32H50.*:ADC\\d*_COMMON:.*", ("adccommon", "h50", "ADC_COMMON")),
    ("STM32H5.*:ADC\\d*_COMMON:.*", ("adccommon", "h5", "ADC_COMMON")),
    ("STM32H7.*:ADC\\d*_COMMON:.*", ("adccommon", "v4", "ADC_COMMON")),
    ("STM32F373.*:SDADC:.*", ("sdadc", "v1", "SDADC")),
    ("STM32F301.*:SDADC:.*", ("sdadc", "v1", "SDADC")),
    ("STM32G4.*:OPAMP:G4_tsmc90_fastOpamp", ("opamp", "g4", "OPAMP")),
    ("STM32F3.*:OPAMP:tsmc018_ull_opamp_v1_0", ("opamp", "f3", "OPAMP")),
    ("STM32H7.*:OPAMP:.*", ("opamp", "h_v1", "OPAMP")),
    ("STM32H5.*:OPAMP:.*", ("opamp", "h_v2", "OPAMP")),
    ("STM32U0.*:OPAMP:.*", ("opamp", "u0", "OPAMP")),
    (".*:DCMI:.*", ("dcmi", "v1", "DCMI")),
    ("STM32C0.*:SYSCFG:.*", ("syscfg", "c0", "SYSCFG")),
    ("STM32F0.*:SYSCFG:.*", ("syscfg", "f0", "SYSCFG")),
    ("STM32F2.*:SYSCFG:.*", ("syscfg", "f2", "SYSCFG")),
    ("STM32F3.*:SYSCFG:.*", ("syscfg", "f3", "SYSCFG")),
    ("STM32F4.*:SYSCFG:.*", ("syscfg", "f4", "SYSCFG")),
    ("STM32F7.*:SYSCFG:.*", ("syscfg", "f7", "SYSCFG")),
    ("STM32L0.*:SYSCFG:.*", ("syscfg", "l0", "SYSCFG")),
    ("STM32L1.*:SYSCFG:.*", ("syscfg", "l1", "SYSCFG")),
    ("STM32L4.*:SYSCFG:.*", ("syscfg", "l4", "SYSCFG")),
    ("STM32L5.*:SYSCFG:.*", ("syscfg", "l5", "SYSCFG")),
    ("STM32G0.*:SYSCFG:.*", ("syscfg", "g0", "SYSCFG")),
    ("STM32G4.*:SYSCFG:.*", ("syscfg", "g4", "SYSCFG")),
    ("STM32H7[RS].*:SYSCFG:.*", ("syscfg", "h7rs", "SYSCFG")),
    (
        "STM32H7(45|47|55|57|42|43|53|50).*:SYSCFG:.*",
        ("syscfg", "h7od", "SYSCFG"),
    ),
    ("STM32H7.*:SYSCFG:.*", ("syscfg", "h7", "SYSCFG")),
    ("STM32U0.*:SYSCFG:.*", ("syscfg", "u0", "SYSCFG")),
    ("STM32U5.*:SYSCFG:.*", ("syscfg", "u5", "SYSCFG")),
    ("STM32WBA.*:SYSCFG:.*", ("syscfg", "wba", "SYSCFG")),
    ("STM32WB.*:SYSCFG:.*", ("syscfg", "wb", "SYSCFG")),
    ("STM32WL5.*:SYSCFG:.*", ("syscfg", "wl5", "SYSCFG")),
    ("STM32WLE.*:SYSCFG:.*", ("syscfg", "wle", "SYSCFG")),
    ("STM32H50.*:SYSCFG:.*", ("syscfg", "h50", "SYSCFG")),
    ("STM32H5.*:SYSCFG:.*", ("syscfg", "h5", "SYSCFG")),
    (".*:IWDG:iwdg1_v1_1", ("iwdg", "v1", "IWDG")),
    (".*:IWDG:iwdg1_v2_0", ("iwdg", "v2", "IWDG")),
    (".*:IWDG:iwdg1_v3_0", ("iwdg", "v3", "IWDG")),
    (".*:WWDG:wwdg1_v1_0", ("wwdg", "v1", "WWDG")),
    (".*:WWDG:wwdg1_v2_0", ("wwdg", "v2", "WWDG")),
    (".*:JPEG:jpeg1_v1_0", ("jpeg", "v1", "JPEG")),
    (".*:LTDC:lcdtft1_v1_0", ("ltdc", "v1", "LTDC")),
    (".*:LTDC:lcdtft1_v1_1", ("ltdc", "v1", "LTDC")),
    (".*:LTDC:lcdtft2_v1_0", ("ltdc", "v1", "LTDC")),
    (".*:DSIHOST:dsihost1_v1_0", ("dsihost", "v1", "DSIHOST")),
    (".*:DSIHOST:dsihost1_v1_0_SHARK", ("dsihost", "v1", "DSIHOST")),
    (".*:DSIHOST:dsihost1_v2_0", ("dsihost", "v2", "DSIHOST")),
    (".*:DSIHOST:dsihost_U5", ("dsihost", "u5", "DSIHOST")),
    (".*:MDIOS:mdios1_v1_0", ("mdios", "v1", "MDIOS")),
    (".*:QUADSPI:.*", ("quadspi", "v1", "QUADSPI")),
    ("STM32F1.*:BKP.*", ("bkp", "v1", "BKP")),
    (".*:RTC:rtc1_v1_1", ("rtc", "v1", "RTC")),
    ("STM32F0.*:RTC:rtc2_.*", ("rtc", "v2f0", "RTC")),
    ("STM32F2.*:RTC:rtc2_.*", ("rtc", "v2f2", "RTC")),
    ("STM32F3.*:RTC:rtc2_.*", ("rtc", "v2f3", "RTC")),
    ("STM32F4.*:RTC:rtc2_.*", ("rtc", "v2f4", "RTC")),
    ("STM32F7.*:RTC:rtc2_.*", ("rtc", "v2f7", "RTC")),
    ("STM32H7.*:RTC:rtc2_.*", ("rtc", "v2h7", "RTC")),
    ("STM32L0.*:RTC:rtc2_.*", ("rtc", "v2l0", "RTC")),
    ("STM32L1.*:RTC:rtc2_.*", ("rtc", "v2l1", "RTC")),
    ("STM32L4.*:RTC:rtc2_.*", ("rtc", "v2l4", "RTC")),
    ("STM32L5.*:RTC:rtc2_.*", ("rtc", "v3l5", "RTC")),
    ("STM32WBA.*:RTC:rtc2_.*", ("rtc", "v3u5", "RTC")),
    ("STM32WB.*:RTC:rtc2_.*", ("rtc", "v2wb", "RTC")),
    ("STM32H5.*:RTC:rtc2_.*", ("rtc", "v3u5", "RTC")),
    ("STM32U5.*:RTC:rtc2_.*", ("rtc", "v3u5", "RTC")), // Cube says v2, but it's v3 with security stuff
    (".*:RTC:rtc3_v1_0", ("rtc", "v3", "RTC")),
    (".*:RTC:rtc3_v1_1", ("rtc", "v3", "RTC")),
    (".*:RTC:rtc3_v2_0", ("rtc", "v3", "RTC")),
    (".*:RTC:rtc3_v3_0", ("rtc", "v3", "RTC")),
    (".*:RTC:rtc3_v3_5", ("rtc", "v3", "RTC")),
    (".*:SAI:sai1_v1_0", ("sai", "v1", "SAI")),
    (".*:SAI:sai1_v1_1", ("sai", "v2", "SAI")),
    (".*:SAI:sai1_v1_2", ("sai", "v2", "SAI")),
    (".*:SAI:sai1_v2_0", ("sai", "v1", "SAI")),
    (".*:SAI:sai1_H7", ("sai", "v3_4pdm", "SAI")),
    (".*:SAI:sai1_v2_1", ("sai", "v4_4pdm", "SAI")),
    (r"STM32[HU]5.*:SAI\d?:.*", ("sai", "v4_2pdm", "SAI")),
    (r"STM32L5.*:SAI\d?:.*", ("sai", "v3_2pdm", "SAI")),
    (".*:SDIO:sdmmc_v1_2", ("sdmmc", "v1", "SDMMC")),
    (".*:SDMMC:sdmmc_v1_3", ("sdmmc", "v1", "SDMMC")),
    (".*:SPDIFRX:spdifrx1_v1_0", ("spdifrx", "v1", "SPDIFRX")),
    (".*:SPDIFRX:spdifrx1_H7", ("spdifrx", "h7", "SPDIFRX")),
    // # USB
    ("STM32(F1|L1).*:USB:.*", ("usb", "v1", "USB")),
    ("STM32(F1|L1).*:USBRAM:.*", ("usbram", "16x1_512", "USBRAM")),
    ("STM32F30[23].[BC].*:USB:.*", ("usb", "v1", "USB")),
    ("STM32F30[23].[BC].*:USBRAM:.*", ("usbram", "16x1_512", "USBRAM")),
    ("STM32F30[23].[68DE].*:USB:.*", ("usb", "v2", "USB")),
    ("STM32F30[23].[68DE].*:USBRAM:.*", ("usbram", "16x2_1024", "USBRAM")),
    ("STM32F373.*:USB:.*", ("usb", "v1", "USB")),
    ("STM32F373.*:USBRAM:.*", ("usbram", "16x2_512", "USBRAM")),
    ("STM32(F0|L[045]|G4|WB).*:USB:.*", ("usb", "v3", "USB")),
    ("STM32(F0|L[045]|G4|WB).*:USBRAM:.*", ("usbram", "16x2_1024", "USBRAM")),
    ("STM32(G0|H5|U5|U0).*:USB:.*", ("usb", "v4", "USB")),
    ("STM32(G0|H5|U5).*:USBRAM:.*", ("usbram", "32_2048", "USBRAM")),
    ("STM32U0.*:USBRAM:.*", ("usbram", "32_1024", "USBRAM")),
    // # USB OTG
    (".*:USB_OTG_FS:otgfs1_.*", ("otg", "v1", "OTG")),
    (".*:USB_OTG_HS:otghs1_.*", ("otg", "v1", "OTG")),
    ("STM32C0.*:RCC:.*", ("rcc", "c0", "RCC")),
    ("STM32F030.[46].*:RCC:.*", ("rcc", "f0v1", "RCC")),
    ("STM32F05[128].*:RCC:.*", ("rcc", "f0v1", "RCC")),
    ("STM32F030.8.*:RCC:.*", ("rcc", "f0v2", "RCC")),
    ("STM32F03[128].*:RCC:.*", ("rcc", "f0v2", "RCC")),
    ("STM32F030.C.*:RCC:.*", ("rcc", "f0v3", "RCC")),
    ("STM32F070.[6B].*:RCC:.*", ("rcc", "f0v3", "RCC")),
    ("STM32F0[479][128].*:RCC:.*", ("rcc", "f0v4", "RCC")),
    ("STM32F100.*:RCC:.*", ("rcc", "f100", "RCC")),
    ("STM32F10[123].*:RCC:.*", ("rcc", "f1", "RCC")),
    ("STM32F10[57].*:RCC:.*", ("rcc", "f1cl", "RCC")),
    ("STM32F2.*:RCC:.*", ("rcc", "f2", "RCC")),
    ("STM32F37.*:RCC:.*", ("rcc", "f37", "RCC")),
    ("STM32F30[23].[BC].*:RCC:.*", ("rcc", "f3v1", "RCC")),
    ("STM32F358.C.*:RCC:.*", ("rcc", "f3v1", "RCC")),
    ("STM32F30[23].[DE].*:RCC:.*", ("rcc", "f3v3", "RCC")),
    ("STM32F398.E.*:RCC:.*", ("rcc", "f3v3", "RCC")),
    ("STM32F3.*:RCC:.*", ("rcc", "f3v2", "RCC")),
    ("STM32F410.*:RCC:.*", ("rcc", "f410", "RCC")),
    ("STM32F4.*:RCC:.*", ("rcc", "f4", "RCC")),
    ("STM32F7.*:RCC:.*", ("rcc", "f7", "RCC")),
    ("STM32G0.*:RCC:.*", ("rcc", "g0", "RCC")),
    ("STM32G4.*:RCC:.*", ("rcc", "g4", "RCC")),
    ("STM32H7[RS].*:RCC:.*", ("rcc", "h7rs", "RCC")),
    ("STM32H7[AB].*:RCC:.*", ("rcc", "h7ab", "RCC")),
    ("STM32H7(42|43|53|50).*:RCC:.*", ("rcc", "h7rm0433", "RCC")),
    ("STM32H7.*:RCC:.*", ("rcc", "h7", "RCC")),
    ("STM32L0.[23].*:RCC:.*", ("rcc", "l0_v2", "RCC")),
    ("STM32L0.*:RCC:.*", ("rcc", "l0", "RCC")),
    ("STM32L1.*:RCC:.*", ("rcc", "l1", "RCC")),
    ("STM32L4[PQRS].*:RCC:.*", ("rcc", "l4plus", "RCC")),
    ("STM32L4.*:RCC:.*", ("rcc", "l4", "RCC")),
    ("STM32L5.*:RCC:.*", ("rcc", "l5", "RCC")),
    ("STM32U0.*:RCC:.*", ("rcc", "u0", "RCC")),
    ("STM32U5.*:RCC:.*", ("rcc", "u5", "RCC")),
    ("STM32H50.*:RCC:.*", ("rcc", "h50", "RCC")),
    ("STM32H5.*:RCC:.*", ("rcc", "h5", "RCC")),
    ("STM32WBA.*:RCC:.*", ("rcc", "wba", "RCC")),
    ("STM32WB.*:RCC:.*", ("rcc", "wb", "RCC")),
    ("STM32WL5.*:RCC:.*", ("rcc", "wl5", "RCC")),
    ("STM32WLE.*:RCC:.*", ("rcc", "wle", "RCC")),
    ("STM32F1.*:SPI[1234]:.*", ("spi", "f1", "SPI")),
    ("STM32F3.*:SPI[1234]:.*", ("spi", "v2", "SPI")),
    ("STM32F1.*:AFIO:.*", ("afio", "f1", "AFIO")),
    ("STM32WBA.*:EXTI:.*", ("exti", "l5", "EXTI")),
    ("STM32L5.*:EXTI:.*", ("exti", "l5", "EXTI")),
    ("STM32C0.*:EXTI:.*", ("exti", "c0", "EXTI")),
    ("STM32G0.*:EXTI:.*", ("exti", "g0", "EXTI")),
    ("STM32H7.*:EXTI:.*", ("exti", "h7", "EXTI")),
    ("STM32U0.*:EXTI:.*", ("exti", "u0", "EXTI")),
    ("STM32U5.*:EXTI:.*", ("exti", "u5", "EXTI")),
    ("STM32WB.*:EXTI:.*", ("exti", "w", "EXTI")),
    ("STM32WL5.*:EXTI:.*", ("exti", "w", "EXTI")),
    ("STM32WLE.*:EXTI:.*", ("exti", "wle", "EXTI")),
    ("STM32H50.*:EXTI:.*", ("exti", "h50", "EXTI")),
    ("STM32H5.*:EXTI:.*", ("exti", "h5", "EXTI")),
    (".*:EXTI:.*", ("exti", "v1", "EXTI")),
    ("STM32F0.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32L0.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32L4.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32L5.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32G0.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32G4.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32U5.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32U0.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32H5.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32H7.*:CRS:.*", ("crs", "v1", "CRS")),
    ("STM32WB.*:CRS:.*", ("crs", "v1", "CRS")),
    (".*SDMMC:sdmmc2_v1_0", ("sdmmc", "v2", "SDMMC")),
    (".*SDMMC:sdmmc2_v2_1", ("sdmmc", "v2", "SDMMC")),
    ("STM32C0.*:PWR:.*", ("pwr", "c0", "PWR")),
    ("STM32G0.*:PWR:.*", ("pwr", "g0", "PWR")),
    ("STM32G4.*:PWR:.*", ("pwr", "g4", "PWR")),
    ("STM32H7[RS].*:PWR:.*", ("pwr", "h7rs", "PWR")),
    ("STM32H7(45|47|55|57).*:PWR:.*", ("pwr", "h7rm0399", "PWR")),
    ("STM32H7(42|43|53|50).*:PWR:.*", ("pwr", "h7rm0433", "PWR")),
    ("STM32H7(23|25|33|35|30).*:PWR:.*", ("pwr", "h7rm0468", "PWR")),
    ("STM32H7(A3|B0|B3).*:PWR:.*", ("pwr", "h7rm0455", "PWR")),
    ("STM32F0.0.*:PWR:.*", ("pwr", "f0x0", "PWR")),
    ("STM32F0.*:PWR:.*", ("pwr", "f0", "PWR")),
    ("STM32F1.*:PWR:.*", ("pwr", "f1", "PWR")),
    ("STM32F2.*:PWR:.*", ("pwr", "f2", "PWR")),
    ("STM32F3.*:PWR:.*", ("pwr", "f3", "PWR")),
    ("STM32F4.*:PWR:.*", ("pwr", "f4", "PWR")),
    ("STM32F7.*:PWR:.*", ("pwr", "f7", "PWR")),
    ("STM32L0.*:PWR:.*", ("pwr", "l0", "PWR")),
    ("STM32L1.*:PWR:.*", ("pwr", "l1", "PWR")),
    ("STM32L4.*:PWR:.*", ("pwr", "l4", "PWR")),
    ("STM32L5.*:PWR:.*", ("pwr", "l5", "PWR")),
    ("STM32U0.*:PWR:.*", ("pwr", "u0", "PWR")),
    ("STM32U5.*:PWR:.*", ("pwr", "u5", "PWR")),
    ("STM32WL.*:PWR:.*", ("pwr", "wl5", "PWR")),
    ("STM32WBA.*:PWR:.*", ("pwr", "wba", "PWR")),
    ("STM32WB[35]5.*:PWR:.*", ("pwr", "wb55", "PWR")),
    ("STM32WB.*:PWR:.*", ("pwr", "wb", "PWR")),
    ("STM32H50.*:PWR:.*", ("pwr", "h50", "PWR")),
    ("STM32H5.*:PWR:.*", ("pwr", "h5", "PWR")),
    ("STM32H7[RS].*:FLASH:.*", ("flash", "h7rs", "FLASH")),
    ("STM32H7(A3|B3|B0).*:FLASH:.*", ("flash", "h7ab", "FLASH")),
    ("STM32H7.*:FLASH:.*", ("flash", "h7", "FLASH")),
    ("STM32F0.*:FLASH:.*", ("flash", "f0", "FLASH")),
    ("STM32F1.*:FLASH:.*", ("flash", "f1", "FLASH")),
    ("STM32F2.*:FLASH:.*", ("flash", "f2", "FLASH")),
    ("STM32F3.*:FLASH:.*", ("flash", "f3", "FLASH")),
    ("STM32F4.*:FLASH:.*", ("flash", "f4", "FLASH")),
    ("STM32F7.*:FLASH:.*", ("flash", "f7", "FLASH")),
    ("STM32L0.*:FLASH:.*", ("flash", "l0", "FLASH")),
    ("STM32L1.*:FLASH:.*", ("flash", "l1", "FLASH")),
    ("STM32L4.*:FLASH:.*", ("flash", "l4", "FLASH")),
    ("STM32L5.*:FLASH:.*", ("flash", "l5", "FLASH")),
    ("STM32U0.*:FLASH:.*", ("flash", "u0", "FLASH")),
    ("STM32U5.*:FLASH:.*", ("flash", "u5", "FLASH")),
    ("STM32WBA.*:FLASH:.*", ("flash", "wba", "FLASH")),
    ("STM32WB.*:FLASH:.*", ("flash", "wb", "FLASH")),
    ("STM32WL.*:FLASH:.*", ("flash", "wl", "FLASH")),
    ("STM32C0.*:FLASH:.*", ("flash", "c0", "FLASH")),
    ("STM32G0.0.*:FLASH:.*", ("flash", "g0x0", "FLASH")),
    ("STM32G0.1.*:FLASH:.*", ("flash", "g0x1", "FLASH")),
    ("STM32G4(3|4).*:FLASH:.*", ("flash", "g4c2", "FLASH")),
    ("STM32G4(7|8).*:FLASH:.*", ("flash", "g4c3", "FLASH")),
    ("STM32G4(9|A).*:FLASH:.*", ("flash", "g4c4", "FLASH")),
    ("STM32H50.*:FLASH:.*", ("flash", "h50", "FLASH")),
    ("STM32H5.*:FLASH:.*", ("flash", "h5", "FLASH")),
    ("STM32F107.*:ETH:.*", ("eth", "v1a", "ETH")),
    ("STM32F[24].*:ETH:.*", ("eth", "v1b", "ETH")),
    ("STM32F7.*:ETH:.*", ("eth", "v1c", "ETH")),
    ("STM32H.*:ETH:.*", ("eth", "v2", "ETH")),
    ("STM32F4[23][79].*:FMC:.*", ("fmc", "v1x3", "FMC")),
    ("STM32F446.*:FMC:.*", ("fmc", "v2x1", "FMC")),
    ("STM32F469.*:FMC:.*", ("fmc", "v2x1", "FMC")),
    ("STM32F7.*:FMC:.*", ("fmc", "v2x1", "FMC")),
    ("STM32H7.*:FMC:.*", ("fmc", "v3x1", "FMC")),
    ("STM32H5.*:FMC:.*", ("fmc", "v4", "FMC")),
    ("STM32F100.*:FSMC:.*", ("fsmc", "v1x0", "FSMC")),
    ("STM32F10[12357].*:FSMC:.*", ("fsmc", "v1x3", "FSMC")),
    ("STM32F2.*:FSMC:.*", ("fsmc", "v1x3", "FSMC")),
    ("STM32F3.*:FSMC:.*", ("fsmc", "v2x3", "FSMC")),
    ("STM32F412.*:FSMC:.*", ("fsmc", "v1x0", "FSMC")),
    ("STM32F4[12]3.*:FSMC:.*", ("fsmc", "v1x0", "FSMC")),
    ("STM32F4[01]5.*:FSMC:.*", ("fsmc", "v1x3", "FSMC")),
    ("STM32F4[01]7.*:FSMC:.*", ("fsmc", "v1x3", "FSMC")),
    ("STM32L1.*:FSMC:.*", ("fsmc", "v1x0", "FSMC")),
    ("STM32L4.*:FSMC:.*", ("fsmc", "v3x1", "FSMC")),
    ("STM32G4.*:FSMC:.*", ("fsmc", "v4x1", "FSMC")),
    ("STM32L5.*:FSMC:.*", ("fsmc", "v4x1", "FSMC")),
    ("STM32U5.*:FSMC:.*", ("fsmc", "v5x1", "FSMC")),
    //// TIM mapping starts here ////
    //
    // Note:
    // AN4013 for the full tables of TIMs
    // AN4013 Rev: 10, Date: 12-Jan-2023
    //
    //
    // AN4013 Table 2: STM32Fx serials
    // Override for STM32Fx serials
    ("STM32F1.*:TIM(2|5):.*", ("timer", "v1", "TIM_4CH")),
    // Normal STM32Fx serials
    ("STM32F.*:TIM(1|8|20):.*", ("timer", "v1", "TIM_ADV4CH")),
    ("STM32F.*:TIM(2|5):.*", ("timer", "v1", "TIM_32BIT")),
    ("STM32F.*:TIM(3|4|19):.*", ("timer", "v1", "TIM_4CH")),
    ("STM32F.*:TIM(6|7|18):.*", ("timer", "v1", "TIM_BASIC")),
    ("STM32F.*:TIM(10|11|13|14):.*", ("timer", "v1", "TIM_1CH")),
    ("STM32F.*:TIM(9|12):.*", ("timer", "v1", "TIM_2CH")),
    ("STM32F.*:TIM15:.*", ("timer", "v1", "TIM_ADV2CH")),
    ("STM32F.*:TIM(16|17):.*", ("timer", "v1", "TIM_ADV1CH")),
    ("STM32F.*:HRTIM:.*", ("hrtim", "v1", "HRTIM")),
    // LPTIM for STM32Fx serials
    ("STM32(F4|F7).*:LPTIM.*:.*", ("lptim", "v1a", "LPTIM")),
    // AN4013 Table 3: STM32Lx serials
    // Override for STM32L0 serial
    ("STM32L0.*:TIM(2|3):.*", ("timer", "l0", "TIM_4CH")),
    ("STM32L0.*:TIM(6|7):.*", ("timer", "l0", "TIM_BASIC")),
    ("STM32L0.*:TIM(21|22):.*", ("timer", "l0", "TIM_2CH")),
    // Override for STM32L1 serials
    ("STM32L1.*:TIM2:.*", ("timer", "v1", "TIM_4CH")),
    // Normal STM32Lx serials
    ("STM32L.*:TIM(1|8):.*", ("timer", "v1", "TIM_ADV4CH")),
    ("STM32L.*:TIM(2|5):.*", ("timer", "v1", "TIM_32BIT")),
    ("STM32L.*:TIM(3|4):.*", ("timer", "v1", "TIM_4CH")),
    ("STM32L.*:TIM(6|7):.*", ("timer", "v1", "TIM_BASIC")),
    ("STM32L.*:TIM(10|11):.*", ("timer", "v1", "TIM_1CH")),
    ("STM32L.*:TIM(9|21|22):.*", ("timer", "v1", "TIM_2CH")),
    ("STM32L.*:TIM15:.*", ("timer", "v1", "TIM_ADV2CH")),
    ("STM32L.*:TIM(16|17):.*", ("timer", "v1", "TIM_ADV1CH")),
    // LPTIM for STM32Lx
    ("STM32L5.*:LPTIM.*:.*", ("lptim", "v1c", "LPTIM")),
    ("STM32L4[PQRS].*:LPTIM.*:.*", ("lptim", "v1b", "LPTIM")),
    ("STM32L4[^PQRS].*:LPTIM.*:.*", ("lptim", "v1a", "LPTIM")),
    ("STM32L0.*:LPTIM.*:.*", ("lptim", "v1", "LPTIM")),
    // AN4013 Table 4: STM32Gx/Hx/Ux/Wx (and Cx) serials
    // timer_v2 for STM32Gx/Hx/Ux/Wx (and Cx) serials
    ("STM32U5.*:TIM(3|4):.*", ("timer", "v2", "TIM_32BIT")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM(1|8|20):.*", ("timer", "v2", "TIM_ADV4CH")),
    (
        "STM32(G4|H5|U0|U5|WBA).*:TIM(2|5|23|24):.*",
        ("timer", "v2", "TIM_32BIT"),
    ),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM(3|4):.*", ("timer", "v2", "TIM_4CH")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM(6|7):.*", ("timer", "v2", "TIM_BASIC")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM(13|14):.*", ("timer", "v2", "TIM_1CH")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM12:.*", ("timer", "v2", "TIM_2CH")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM15:.*", ("timer", "v2", "TIM_ADV2CH")),
    ("STM32(G4|H5|U0|U5|WBA).*:TIM(16|17):.*", ("timer", "v2", "TIM_ADV1CH")),
    ("STM32G4.*:HRTIM1:.*", ("hrtim", "v2", "HRTIM")),
    // timer_v1 for STM32Gx/Hx/Ux/Wx (and Cx) serials
    ("STM32(C|G0|H7|WB|WL).*:TIM(1|8|20):.*", ("timer", "v1", "TIM_ADV4CH")),
    ("STM32(C|G0|H7|WB|WL).*:TIM(2|5|23|24):.*", ("timer", "v1", "TIM_32BIT")),
    ("STM32(C|G0|H7|WB|WL).*:TIM(3|4):.*", ("timer", "v1", "TIM_4CH")),
    ("STM32(C|G0|H7|WB|WL).*:TIM(6|7):.*", ("timer", "v1", "TIM_BASIC")),
    ("STM32(C|G0|H7|WB|WL).*:TIM(13|14):.*", ("timer", "v1", "TIM_1CH")),
    ("STM32(C|G0|H7|WB|WL).*:TIM12:.*", ("timer", "v1", "TIM_2CH")),
    ("STM32(C|G0|H7|WB|WL).*:TIM15:.*", ("timer", "v1", "TIM_ADV2CH")),
    ("STM32(C|G0|H7|WB|WL).*:TIM(16|17):.*", ("timer", "v1", "TIM_ADV1CH")),
    ("STM32[CGHUW].*:HRTIM1?:.*", ("hrtim", "v1", "HRTIM")),
    // LPTIM for STM32Gx/Hx/Ux/Wx (and Cx) serials
    ("STM32U0.*:LPTIM.*:.*", ("lptim", "v2b", "LPTIM")),
    ("STM32(H5|U5|WBA).*:LPTIM[12356]:.*", ("lptim", "v2a", "LPTIM")),
    ("STM32(H5|U5).*:LPTIM4:.*", ("lptim", "v2a", "LPTIM_BASIC")),
    ("STM32WL.*:LPTIM.*:.*", ("lptim", "v1c", "LPTIM")),
    ("STM32H7.*:LPTIM.*:.*", ("lptim", "v1b_h7", "LPTIM")),
    ("STM32G4.*:LPTIM.*:.*", ("lptim", "v1b_g4", "LPTIM")),
    ("STM32(G0|WB).*:LPTIM.*:.*", ("lptim", "v1b", "LPTIM")),
    //
    //// TIM mapping ends here ////
    ("STM32F0.*:DBGMCU:.*", ("dbgmcu", "f0", "DBGMCU")),
    ("STM32F1.*:DBGMCU:.*", ("dbgmcu", "f1", "DBGMCU")),
    ("STM32F2.*:DBGMCU:.*", ("dbgmcu", "f2", "DBGMCU")),
    ("STM32F3.*:DBGMCU:.*", ("dbgmcu", "f3", "DBGMCU")),
    ("STM32F4.*:DBGMCU:.*", ("dbgmcu", "f4", "DBGMCU")),
    ("STM32F7.*:DBGMCU:.*", ("dbgmcu", "f7", "DBGMCU")),
    ("STM32C0.*:DBGMCU:.*", ("dbgmcu", "c0", "DBGMCU")),
    ("STM32G0.*:DBGMCU:.*", ("dbgmcu", "g0", "DBGMCU")),
    ("STM32G4.*:DBGMCU:.*", ("dbgmcu", "g4", "DBGMCU")),
    ("STM32H5.*:DBGMCU:.*", ("dbgmcu", "h5", "DBGMCU")),
    ("STM32H7.*:DBGMCU:.*", ("dbgmcu", "h7", "DBGMCU")),
    ("STM32L0.*:DBGMCU:.*", ("dbgmcu", "l0", "DBGMCU")),
    ("STM32L1.*:DBGMCU:.*", ("dbgmcu", "l1", "DBGMCU")),
    ("STM32L4.*:DBGMCU:.*", ("dbgmcu", "l4", "DBGMCU")),
    ("STM32L5.*:DBGMCU:.*", ("dbgmcu", "l5", "DBGMCU")),
    ("STM32U5.*:DBGMCU:.*", ("dbgmcu", "u5", "DBGMCU")),
    ("STM32U0.*:DBGMCU:.*", ("dbgmcu", "u0", "DBGMCU")),
    ("STM32WBA.*:DBGMCU:.*", ("dbgmcu", "wba", "DBGMCU")),
    ("STM32WB.*:DBGMCU:.*", ("dbgmcu", "wb", "DBGMCU")),
    ("STM32WL.*:DBGMCU:.*", ("dbgmcu", "wl", "DBGMCU")),
    ("STM32F1.*:GPIO.*", ("gpio", "v1", "GPIO")),
    (".*:GPIO.*", ("gpio", "v2", "GPIO")),
    (".*:IPCC:v1_0", ("ipcc", "v1", "IPCC")),
    ("STM32H7(4|5)(5|7).*:HSEM:.*", ("hsem", "v1", "HSEM")),
    ("STM32WB55.*:HSEM:.*", ("hsem", "v1", "HSEM")),
    ("STM32H735.*:HSEM:.*", ("hsem", "v2", "HSEM")),
    ("STM32H7B3.*:HSEM:.*", ("hsem", "v2", "HSEM")),
    ("STM32H753.*:HSEM:.*", ("hsem", "v2", "HSEM")),
    ("STM32H743.*:HSEM:.*", ("hsem", "v2", "HSEM")),
    ("STM32WL5.*:HSEM:.*", ("hsem", "v3", "HSEM")),
    ("STM32WLE.*:HSEM:.*", ("hsem", "v4", "HSEM")),
    (".*:DMAMUX.*", ("dmamux", "v1", "DMAMUX")),
    (r".*:GPDMA\d?:.*", ("gpdma", "v1", "GPDMA")), // TODO there's multiple versions for with+without trustzone.
    (r".*:HPDMA\d?:.*", ("gpdma", "v1", "GPDMA")), // TODO it has a few more bits like DWX
    (r".*:LPDMA\d?:.*", ("lpdma", "v1", "LPDMA")),
    (r".*:BDMA\d?:.*", ("bdma", "v1", "DMA")),
    ("STM32H7.*:DMA2D:DMA2D:dma2d1_v1_0", ("dma2d", "v2", "DMA2D")),
    (".*:DMA2D:dma2d1_v1_0", ("dma2d", "v1", "DMA2D")),
    ("STM32L4[PQRS].*:DMA.*", ("bdma", "v1", "DMA")), // L4+
    ("STM32L[04].*:DMA.*", ("bdma", "v2", "DMA")),    // L0, L4 non-plus (since plus is handled above)
    ("STM32F030.C.*:DMA.*", ("bdma", "v2", "DMA")),   // Weird F0
    ("STM32F09.*:DMA.*", ("bdma", "v2", "DMA")),      // Weird F0
    ("STM32F[247].*:DMA.*", ("dma", "v2", "DMA")),
    ("STM32H7.*:DMA.*", ("dma", "v1", "DMA")),
    (".*:DMA.*", ("bdma", "v1", "DMA")),
    (".*:CAN:bxcan1_v1_1.*", ("can", "bxcan", "CAN")),
    ("STM32H7.*:FDCAN:fdcan1_v1_[01].*", ("can", "fdcan_h7", "FDCAN")),
    (".*:FDCAN:fdcan1_v1_[01].*", ("can", "fdcan_v1", "FDCAN")),
    ("STM32H7.*:FDCANRAM.*", ("fdcanram", "h7", "FDCANRAM")),
    (".*:FDCANRAM.*", ("fdcanram", "v1", "FDCANRAM")),
    ("STM32F[124].*:CRC:.*", ("crc", "v1", "CRC")),
    ("STM32L1.*:CRC:.*", ("crc", "v1", "CRC")),
    ("STM32F0.*:CRC:.*", ("crc", "v2", "CRC")),
    ("STM32F[37].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32G[04].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32H[57].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32L[045].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32W[BL].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32C[0].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32U[0].*:CRC:.*", ("crc", "v3", "CRC")),
    ("STM32U[5].*:CRC:.*", ("crc", "v3", "CRC")),
    (".*:LCD:lcdc1_v1.0.*", ("lcd", "v1", "LCD")),
    (".*:LCD:lcdc1_v1.2.*", ("lcd", "v2", "LCD")),
    (".*:LCD:lcdc1_v1.3.*", ("lcd", "v2", "LCD")),
    (".*:LCD:lcdc1_v1.4.*", ("lcd", "v2", "LCD")),
    (".*:UID:.*", ("uid", "v1", "UID")),
    ("STM32H5.*:UCPD:.*", ("ucpd", "h5", "UCPD")),
    (".*:UCPD:.*", ("ucpd", "v1", "UCPD")),
    ("STM32G0.*:TAMP:.*", ("tamp", "g0", "TAMP")),
    ("STM32G4.*:TAMP:.*", ("tamp", "g4", "TAMP")),
    ("STM32H5.*:TAMP:.*", ("tamp", "h5", "TAMP")),
    ("STM32L5.*:TAMP:.*", ("tamp", "l5", "TAMP")),
    ("STM32U5.*:TAMP:.*", ("tamp", "u5", "TAMP")),
    ("STM32WL.*:TAMP:.*", ("tamp", "wl", "TAMP")),
    (".*:OCTOSPIM:OCTOSPIM:.*", ("octospim", "v1", "OCTOSPIM")),
    // it's actually STM32L4+, not STM32L4
    (
        "STM32L4.*:OCTOSPI[12]:OCTOSPI:octospi_v1_0.*",
        ("octospi", "v1", "OCTOSPI"),
    ),
    (
        "STM32H7.*:OCTOSPI[12]:OCTOSPI:octospi_v2_1H7AB.*",
        ("octospi", "v1", "OCTOSPI"),
    ),
    (
        "STM32U5[34].*:OCTOSPI[12]:OCTOSPI:octospi_v1_0L5.*",
        ("octospi", "v1", "OCTOSPI"),
    ),
    (
        "STM32U5[AFG789].*:OCTOSPI[12]:OCTOSPI:octospi1_v3_0.*",
        ("octospi", "v1", "OCTOSPI"),
    ),
    (
        "STM32L5.*:OCTOSPI[12]:OCTOSPI:octospi_v1_0L5.*",
        ("octospi", "v2", "OCTOSPI"),
    ),
    (
        "STM32H5.*:OCTOSPI[12]:OCTOSPI:octospi1_v5_1.*",
        ("octospi", "v2", "OCTOSPI"),
    ),
    ("STM32U5[AFG9]9.*:HSPI1:HSPI:hspi1_v1_0.*", ("hspi", "v1", "HSPI")),
    ("STM32L4.*:GFXMMU:.*", ("gfxmmu", "v1", "GFXMMU")),
    ("STM32U5.*:GFXMMU:.*", ("gfxmmu", "v2", "GFXMMU")),
    ("STM32U5.*:ICACHE:.*", ("icache", "v1_3crr", "ICACHE")),
    ("STM32H50.*:ICACHE:.*", ("icache", "v1_0crr", "ICACHE")),
    ("STM32(L5|H5[67]|WBA).*:ICACHE:.*", ("icache", "v1_4crr", "ICACHE")),
    (".*:CORDIC:.*", ("cordic", "v1", "CORDIC")),
    ("STM32F0.[128].*:TSC:.*", ("tsc", "v1", "TSC")),
    ("STM32F3[07][123].*:TSC:.*", ("tsc", "v1", "TSC")),
    ("STM32WB55.*:TSC:.*", ("tsc", "v2", "TSC")),
    ("STM32WBA.*:TSC:.*", ("tsc", "v1", "TSC")),
    ("STM32L[045].*:TSC:.*", ("tsc", "v3", "TSC")),
    ("STM32U5.*:TSC:.*", ("tsc", "v3", "TSC")),
    ("STM32U0.*:TSC:.*", ("tsc", "v2", "TSC")),
    ("*:VREFINTCAL:.*", ("vrefintcal", "v1", "VREFINTCAL")),
    ("STM32U5.*:ADF[12]:.*", ("adf", "v1", "ADF")),
    (".*:HASH:hash1_v1_0", ("hash", "v1", "HASH")),
    (".*:HASH:hash1_v2_0", ("hash", "v2", "HASH")),
    ("STM32U5.*:HASH:.*", ("hash", "v4", "HASH")),
    ("STM32WBA.*:HASH:.*", ("hash", "v4", "HASH")),
    (".*:HASH:hash1_v2_2", ("hash", "v2", "HASH")),
    (".*:HASH:hash1_v4_0", ("hash", "v3", "HASH")),
    (".*:CRYP:cryp1_v1_0.*", ("cryp", "v1", "CRYP")),
    (".*:CRYP:cryp1_v2_0_H7.*", ("cryp", "v3", "CRYP")),
    (".*:CRYP:cryp1-v2_5", ("cryp", "v4", "CRYP")),
    (".*:CRYP:cryp1_v2_0.*", ("cryp", "v2", "CRYP")),
    ("STM32F41.*:CRYP:cryp1_v2_2.*", ("cryp", "v1", "CRYP")),
    (".*:CRYP:cryp1_v2_2.*", ("cryp", "v2", "CRYP")),
    ("STM32G0.1.*:.*:COMP:.*", ("comp", "v1", "COMP")),
    ("STM32G4.*:.*:COMP:.*", ("comp", "v2", "COMP")),
    ("STM32U0.*:.*:COMP:.*", ("comp", "u0", "COMP")),
    ("STM32WL.*:.*:COMP:.*", ("comp", "v3", "COMP")),
    ("STM32H7[45].*:COMP:.*", ("comp", "h7_b", "COMP")),
    ("STM32H7[AB].*:COMP:.*", ("comp", "h7_a", "COMP")),
    ("STM32H5.*:COMP:.*", ("comp", "h5", "COMP")),
    ("STM32U5[34].*:COMP1:.*", ("comp", "u5", "COMP")),
    ("STM32U5[AFG789].*:COMP[12]:.*", ("comp", "u5", "COMP")),
    ("STM32F373.*:COMP[12]:.*", ("comp", "f3_v1", "COMP")),
    (r".*:.*:DCACHE:.*", ("dcache", "v1", "DCACHE")),
    (".*:.*:PSSI:.*", ("pssi", "v1", "PSSI")),
    (".*:.*:DTS:.*", ("dts", "v1", "DTS")),
    // HDMI_CEC for F1
    (".*:HDMI_CEC:hdmi_cec_v1_1", ("cec", "v1", "CEC")),
    // HDMI_CEC for others
    (".*:HDMI_CEC:hdmi_cec_v2_0", ("cec", "v2", "CEC")),
    ("STM32(L5|L4|G0|WB|WL).*:VREFBUF:.*", ("vrefbuf", "v1", "VREFBUF")),
    ("STM32(H7|U5).*:VREFBUF:.*", ("vrefbuf", "v2a1", "VREFBUF")),
    ("STM32H5.*:VREFBUF:.*", ("vrefbuf", "v2a2", "VREFBUF")),
    ("STM32G4.*:VREFBUF:.*", ("vrefbuf", "v2b", "VREFBUF")),
    ("STM32H5.*:I3C:.*", ("i3c", "v1", "I3C")),
    ("STM32(H5|WBA).*:PKA:.*", ("pka", "v1a", "PKA")),
    ("STM32U5.*:PKA:.*", ("pka", "v1b", "PKA")),
    ("STM32(L5|WL|WB).*:PKA:.*", ("pka", "v1c", "PKA")),
    ("STM32(L4Q|L5|WL|WB).*:PKA:.*", ("pka", "v1c", "PKA")),
    (".*:OTFDEC:.*", ("otfdec", "v1", "OTFDEC")),
]);
