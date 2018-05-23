#[macro_use]
extern crate bitflags;
extern crate embedded_hal as hal;
#[macro_use]
extern crate log;

mod calc;

use calc::Calc;
use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write};
use std::result;

/** BME680 General config */
pub const BME680_POLL_PERIOD_MS: u8 = 10;

/** BME680 I2C addresses */
pub const BME680_I2C_ADDR_PRIMARY: u8 = 0x76;
pub const BME680_I2C_ADDR_SECONDARY: u8 = 0x77;

/** BME680 unique chip identifier */
pub const BME680_CHIP_ID: u8 = 0x61;

/** BME680 coefficients related defines */
pub const BME680_COEFF_SIZE: usize = 41;
pub const BME680_COEFF_ADDR1_LEN: usize = 25;
pub const BME680_COEFF_ADDR2_LEN: usize = 16;

/** BME680 field_x related defines */
pub const BME680_FIELD_LENGTH: usize = 15;
pub const BME680_FIELD_ADDR_OFFSET: u8 = 17;

pub const BME680_SOFT_RESET_CMD: u8 = 0xb6;

pub const BME680_OK: i8 = 0;

/** Errors **/
pub const BME680_E_NULL_PTR: i8 = -1;
pub const BME680_E_COM_FAIL: i8 = -2;
pub const BME680_E_DEV_NOT_FOUND: i8 = -3;
pub const BME680_E_INVALID_LENGTH: i8 = -4;

/** Register map */
/** Other coefficient's address */
pub const BME680_ADDR_RES_HEAT_VAL_ADDR: u8 = 0x00;
pub const BME680_ADDR_RES_HEAT_RANGE_ADDR: u8 = 0x02;
pub const BME680_ADDR_RANGE_SW_ERR_ADDR: u8 = 0x04;
pub const BME680_ADDR_SENS_CONF_START: u8 = 0x5A;
pub const BME680_ADDR_GAS_CONF_START: u8 = 0x64;

pub const BME680_SOFT_RESET_ADDR: u8 = 0xe0;

/** Over-sampling settings */
// TODO replace with enum/flags
pub const BME680_OS_NONE: u8 = 0;
pub const BME680_OS_1X: u8 = 1;
pub const BME680_OS_2X: u8 = 2;
pub const BME680_OS_4X: u8 = 3;
pub const BME680_OS_8X: u8 = 4;
pub const BME680_OS_16X: u8 = 5;

/** Field settings */
pub const BME680_FIELD0_ADDR: u8 = 0x1d;

/** Heater settings */
pub const BME680_RES_HEAT0_ADDR: u8 = 0x5a;
pub const BME680_GAS_WAIT0_ADDR: u8 = 0x64;

/** Sensor configuration registers */
pub const BME680_CONF_HEAT_CTRL_ADDR: u8 = 0x70;
pub const BME680_CONF_ODR_RUN_GAS_NBC_ADDR: u8 = 0x71;
pub const BME680_CONF_OS_H_ADDR: u8 = 0x72;
pub const BME680_MEM_PAGE_ADDR: u8 = 0xf3;
pub const BME680_CONF_T_P_MODE_ADDR: u8 = 0x74;
pub const BME680_CONF_ODR_FILT_ADDR: u8 = 0x75;

/** Coefficient's address */
pub const BME680_COEFF_ADDR1: u8 = 0x89;
pub const BME680_COEFF_ADDR2: u8 = 0xe1;

/** Chip identifier */
pub const BME680_CHIP_ID_ADDR: u8 = 0xd0;

pub const BME680_SLEEP_MODE: u8 = 0;
pub const BME680_FORCED_MODE: u8 = 1;

pub const BME680_RESET_PERIOD: u8 = 10;

pub const BME680_GAS_MEAS_MSK: u8 = 0x30;
pub const BME680_NBCONV_MSK: u8 = 0x0F;
pub const BME680_FILTER_MSK: u8 = 0x1C;
pub const BME680_OST_MSK: u8 = 0xE0;
pub const BME680_OSP_MSK: u8 = 0x1C;
pub const BME680_OSH_MSK: u8 = 0x07;
pub const BME680_HCTRL_MSK: u8 = 0x08;
pub const BME680_RUN_GAS_MSK: u8 = 0x10;
pub const BME680_MODE_MSK: u8 = 0x03;
pub const BME680_RHRANGE_MSK: u8 = 0x30;
pub const BME680_RSERROR_MSK: u8 = 0xf0;
pub const BME680_NEW_DATA_MSK: u8 = 0x80;
pub const BME680_GAS_INDEX_MSK: u8 = 0x0f;
pub const BME680_GAS_RANGE_MSK: u8 = 0x0f;
pub const BME680_GASM_VALID_MSK: u8 = 0x20;
pub const BME680_HEAT_STAB_MSK: u8 = 0x10;
pub const BME680_MEM_PAGE_MSK: u8 = 0x10;
pub const BME680_SPI_RD_MSK: u8 = 0x80;
pub const BME680_SPI_WR_MSK: u8 = 0x7f;
pub const BME680_BIT_H1_DATA_MSK: u8 = 0x0F;

///
/// SPI memory page settings
///
pub const BME680_MEM_PAGE0: u8 = 0x10;

///
/// SPI memory page settings
///
pub const BME680_MEM_PAGE1: u8 = 0x00;

/** Buffer length macro declaration */
pub const BME680_TMP_BUFFER_LENGTH: usize = 40;
pub const BME680_REG_BUFFER_LENGTH: usize = 6;
pub const BME680_FIELD_DATA_LENGTH: usize = 3;
pub const BME680_GAS_REG_BUF_LENGTH: usize = 20;

/* Settings selector */
pub const BME680_OST_SEL: u16 = 1;
pub const BME680_OSP_SEL: u16 = 2;
pub const BME680_OSH_SEL: u16 = 4;
pub const BME680_GAS_MEAS_SEL: u16 = 8;
pub const BME680_FILTER_SEL: u16 = 16;
pub const BME680_HCNTRL_SEL: u16 = 32;
pub const BME680_RUN_GAS_SEL: u16 = 64;
pub const BME680_NBCONV_SEL: u16 = 128;
pub const BME680_GAS_SENSOR_SEL: u16 = BME680_GAS_MEAS_SEL | BME680_RUN_GAS_SEL | BME680_NBCONV_SEL;

#[derive(Debug)]
pub enum Bme680Error<R, W> {
    ///
    /// aka BME680_E_NULL_PTR
    ///
    NulltPtr,
    ///
    /// aka BME680_E_COM_FAIL
    ///
    I2CWrite(W),
    I2CRead(R),
    ///
    /// aka BME680_E_DEV_NOT_FOUND
    ///
    DeviceNotFound,
    ///
    /// aka BME680_E_INVALID_LENGTH
    ///
    InvalidLength,
    ///
    /// Warning aka BME680_W_DEFINE_PWR_MODE
    ///
    DefinePwrMode,
    ///
    /// Warning aka BME680_W_DEFINE_PWR_MODE
    ///
    NoNewData,
    ///
    /// Warning Boundary Check
    ///
    BoundaryCheckFailure(InfoMsg, u8, u8),
}

//impl<I2C> From<<I2C as Read>::Error> for Bme680Error<<I2C as Read>::Error, <I2C as Write>::Error>
//    where I2C: Read + Write {
//    fn from(e: <I2C as Read>::Error) {
//        Bme680Error::I2CRead(e)
//    }
//}

pub type Result<T, R, W> = result::Result<T, Bme680Error<R, W>>;

///
/// Power mode settings
///
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PowerMode {
    SleepMode,
    ForcedMode,
}

impl PowerMode {
    // TODO replace with TryFrom once stabilized
    fn from(power_mode: u8) -> Self {
        match power_mode {
            BME680_SLEEP_MODE => PowerMode::SleepMode,
            BME680_FORCED_MODE => PowerMode::ForcedMode,
            _ => panic!("Unknown power mode: {}", power_mode),
        }
    }

    fn value(&self) -> u8 {
        match self {
            PowerMode::SleepMode => BME680_SLEEP_MODE,
            PowerMode::ForcedMode => BME680_FORCED_MODE,
        }
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct CalibData {
    pub par_h1: u16,
    pub par_h2: u16,
    pub par_h3: i8,
    pub par_h4: i8,
    pub par_h5: i8,
    pub par_h6: u8,
    pub par_h7: i8,
    pub par_gh1: i8,
    pub par_gh2: i16,
    pub par_gh3: i8,
    pub par_t1: u16,
    pub par_t2: i16,
    pub par_t3: i8,
    pub par_p1: u16,
    pub par_p2: i16,
    pub par_p3: i8,
    pub par_p4: i16,
    pub par_p5: i16,
    pub par_p6: i8,
    pub par_p7: i8,
    pub par_p8: i16,
    pub par_p9: i16,
    pub par_p10: u8,
    pub res_heat_range: u8,
    pub res_heat_val: i8,
    pub range_sw_err: u8,
}

impl Clone for CalibData {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct TphSett {
    pub os_hum: Option<OversamplingSetting>,
    pub os_temp: Option<OversamplingSetting>,
    pub os_pres: Option<OversamplingSetting>,
    pub filter: Option<u8>,
}

impl Clone for TphSett {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct GasSett {
    pub nb_conv: u8,
    pub heatr_ctrl: Option<u8>,
    pub run_gas: Option<u8>,
    pub heatr_temp: Option<u16>,
    pub heatr_dur: Option<u16>,
}

impl Clone for GasSett {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, Default, Copy)]
#[repr(C)]
pub struct FieldData {
    pub status: u8,
    pub gas_index: u8,
    pub meas_index: u8,
    pub temperature: i16,
    pub pressure: u32,
    pub humidity: u32,
    pub gas_resistance: u32,
}

impl Clone for FieldData {
    fn clone(&self) -> Self {
        *self
    }
}

impl FieldData {
    pub fn temperature_celsius(&self) -> f32 {
        self.temperature as f32 / 100f32
    }

    pub fn pressure_hpa(&self) -> f32 {
        self.pressure as f32 / 100f32
    }

    pub fn humidity_percent(&self) -> f32 {
        self.humidity as f32 / 1000f32
    }
}

/// TODO - replace naming of "State" with something better
/// aka new_fields - BME680_NEW_DATA_MSK
///
#[derive(Debug)]
pub enum FieldDataState {
    NewData,
    // TODO find better naming to no new data
    NoNewData,
}

/// Infos
bitflags! {
    #[derive(Default)]
    pub struct InfoMsg: u8 {
        const MIN_CORRECTION = 1;
        const MAX_CORRECTION = 2;
    }
}

#[derive(Debug, Default)]
pub struct SensorSettings {
    pub gas_sett: GasSett,
    pub tph_sett: TphSett,
}

bitflags! {
    pub struct DesiredSensorSettings: u16 {
        /// To set temperature oversampling
        const OST_SEL = 1;
        /// To set pressure oversampling.
        const OSP_SEL = 2;

        /// To set humidity oversampling.
        const OSH_SEL = 4;
        /// To set gas measurement setting.
        const GAS_MEAS_SEL = 8;
        /// To set filter setting.
        const FILTER_SEL = 16;
        /// To set humidity control setting.
        const HCNTRL_SEL = 32;
        /// To set run gas setting.
        const RUN_GAS_SEL = 64;
        /// To set NB conversion setting.
        const NBCONV_SEL = 128;
        /// To set all gas sensor related settings
        const GAS_SENSOR_SEL = Self::GAS_MEAS_SEL.bits | Self::RUN_GAS_SEL.bits | Self::NBCONV_SEL.bits;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OversamplingSetting {
    OSNone = 0,
    OS1x = 1,
    OS2x = 2,
    OS4x = 3,
    OS8x = 4,
    OS16x = 5,
}

impl OversamplingSetting {
    // TODO replace with TryFrom once stabilized
    fn from(os: u8) -> OversamplingSetting {
        match os {
            0 => OversamplingSetting::OSNone,
            1 => OversamplingSetting::OS1x,
            2 => OversamplingSetting::OS2x,
            3 => OversamplingSetting::OS4x,
            4 => OversamplingSetting::OS8x,
            5 => OversamplingSetting::OS16x,
            _ => panic!("Unknown oversampling setting: {}", os),
        }
    }
}

pub struct I2CUtil {}

impl I2CUtil {
    pub fn read_byte<I2C>(
        i2c: &mut I2C,
        dev_id: u8,
        reg_addr: u8,
    ) -> Result<u8, <I2C as Read>::Error, <I2C as Write>::Error>
    where
        I2C: Read + Write,
    {
        let mut buf = [0; 1];

        i2c.write(dev_id, &mut [reg_addr])
            .map_err(|e| Bme680Error::I2CWrite(e))?;

        match i2c.read(dev_id, &mut buf) {
            Ok(()) => Ok(buf[0]),
            Err(e) => Err(Bme680Error::I2CRead(e)),
        }
    }

    pub fn read_bytes<I2C>(
        i2c: &mut I2C,
        dev_id: u8,
        reg_addr: u8,
        buf: &mut [u8],
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error>
    where
        I2C: Read + Write,
    {
        i2c.write(dev_id, &mut [reg_addr])
            .map_err(|e| Bme680Error::I2CWrite(e))?;

        match i2c.read(dev_id, buf) {
            Ok(()) => Ok(()),
            Err(e) => Err(Bme680Error::I2CRead(e)),
        }
    }
}

#[repr(C)]
pub struct Bme680_dev<I2C, D> {
    i2c: I2C,
    delay: D,
    dev_id: u8,
    amb_temp: i8,
    calib: CalibData,
    // TODO remove ? as it may not reflect the state of the device
    tph_sett: TphSett,
    // TODO remove ? as it may not reflect the state of the device
    gas_sett: GasSett,
    // TODO remove ? as it may not reflect the state of the device
    power_mode: PowerMode,
}

fn boundary_check<I2C>(
    value: Option<u8>,
    min: u8,
    max: u8,
) -> Result<u8, <I2C as Read>::Error, <I2C as Write>::Error>
where
    I2C: Read + Write,
{
    let mut info_msg: InfoMsg = Default::default();

    // TODO give the nullptr here a different name
    let value = value.unwrap();

    if value < min {
        error!("Boundary check failure max correction, {} < {}", value, min);
        info_msg |= InfoMsg::MIN_CORRECTION;
    }

    if value > max {
        error!("Boundary check failure max correction, {} > {}", value, max);
        info_msg |= InfoMsg::MAX_CORRECTION;
    }

    if !info_msg.is_empty() {
        error!("Boundary check failure");
        return Err(Bme680Error::BoundaryCheckFailure(info_msg, min, max));
    }
    Ok(value)
}

impl<I2C, D> Bme680_dev<I2C, D>
where
    D: DelayMs<u8>,
    I2C: Read + Write,
{
    pub fn soft_reset(
        i2c: &mut I2C,
        delay: &mut D,
        dev_id: u8,
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error> {
        let tmp_buff: [u8; 2] = [BME680_SOFT_RESET_ADDR, BME680_SOFT_RESET_CMD];

        i2c.write(dev_id, &tmp_buff)
            .map_err(|e| Bme680Error::I2CWrite(e))?;

        delay.delay_ms(BME680_RESET_PERIOD);
        Ok(())
    }

    pub fn init(
        mut i2c: I2C,
        mut delay: D,
        dev_id: u8,
        ambient_temperature: i8,
    ) -> Result<Bme680_dev<I2C, D>, <I2C as Read>::Error, <I2C as Write>::Error> {
        Bme680_dev::soft_reset(&mut i2c, &mut delay, dev_id)?;

        debug!("Reading chip id");
        /* Soft reset to restore it to default values*/
        let chip_id = I2CUtil::read_byte::<I2C>(&mut i2c, dev_id, BME680_CHIP_ID_ADDR)?;
        debug!("Chip id: {}", chip_id);

        if chip_id == BME680_CHIP_ID {
            debug!("Reading calib data");
            let calib = Bme680_dev::<I2C, D>::get_calib_data::<I2C>(&mut i2c, dev_id)?;
            debug!("Calib data {:?}", calib);
            let dev = Bme680_dev {
                i2c: i2c,
                delay: delay,
                dev_id: dev_id,
                calib: calib,
                amb_temp: ambient_temperature,
                power_mode: PowerMode::ForcedMode,
                tph_sett: Default::default(),
                gas_sett: Default::default(),
            };
            info!("Finished device init");
            Ok(dev)
        } else {
            error!("Device does not match chip id {}", BME680_CHIP_ID);
            Err(Bme680Error::DeviceNotFound)
        }
    }

    pub fn bme680_set_regs(
        &mut self,
        reg: &[(u8, u8)],
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error> {
        if reg.is_empty() || reg.len() > (BME680_TMP_BUFFER_LENGTH / 2) as usize {
            return Err(Bme680Error::InvalidLength);
        }

        for (reg_addr, reg_data) in reg {
            let tmp_buff: [u8; 2] = [reg_addr.clone(), reg_data.clone()];
            debug!(
                "Setting register reg: {:?} tmp_buf: {:?}",
                reg_addr, tmp_buff
            );
            self.i2c
                .write(self.dev_id, &tmp_buff)
                .map_err(|e| Bme680Error::I2CWrite(e))?;
        }

        Ok(())

        //        self.i2c
        //            .write(self.dev_id, tmp_buff.as_slice())
        //            .map_err(|e| Bme680Error::I2CWrite(e))
    }

    // TODO replace all the NullPtr mess and split this into separate methods
    pub fn set_sensor_settings(
        &mut self,
        desired_settings: DesiredSensorSettings,
        sensor_settings: &SensorSettings,
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error> {
        let tph_sett = sensor_settings.tph_sett;
        let gas_sett = sensor_settings.gas_sett;

        let mut reg = Vec::with_capacity(BME680_REG_BUFFER_LENGTH);
        let intended_power_mode = self.power_mode;

        if desired_settings.contains(DesiredSensorSettings::GAS_MEAS_SEL) {
            debug!("GAS_MEAS_SEL: true");
            self.set_gas_config(gas_sett)?;
        }

        let power_mode = self.power_mode;
        self.set_sensor_mode(power_mode)?;

        /* Selecting the filter */
        if desired_settings.contains(DesiredSensorSettings::FILTER_SEL) {
            let tph_sett_filter = boundary_check::<I2C>(tph_sett.filter, 0, 7)?;
            let mut data =
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_ODR_FILT_ADDR)?;

            debug!("FILTER_SEL: true");
            data = (data as (i32) & !0x1ci32 | tph_sett_filter as (i32) << 2i32 & 0x1ci32) as (u8);
            reg.push((BME680_CONF_ODR_FILT_ADDR, data));
        }

        if desired_settings.contains(DesiredSensorSettings::HCNTRL_SEL) {
            debug!("HCNTRL_SEL: true");
            let gas_sett_heatr_ctrl = boundary_check::<I2C>(gas_sett.heatr_ctrl, 0x0u8, 0x8u8)?;
            let mut data =
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_HEAT_CTRL_ADDR)?;
            data = (data as (i32) & !0x8i32 | gas_sett_heatr_ctrl as (i32) & 0x8) as (u8);
            reg.push((BME680_CONF_HEAT_CTRL_ADDR, data));
        }

        /* Selecting heater T,P oversampling for the sensor */
        if desired_settings
            .contains(DesiredSensorSettings::OST_SEL | DesiredSensorSettings::OSP_SEL)
        {
            let mut data =
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_T_P_MODE_ADDR)?;

            if desired_settings.contains(DesiredSensorSettings::OST_SEL) {
                debug!("OST_SEL: true");
                let tph_sett_os_temp =
                    boundary_check::<I2C>(tph_sett.os_temp.map(|x| x as u8), 0, 5)?;
                data = (data as (i32) & !0xe0i32 | tph_sett_os_temp as (i32) << 5i32 & 0xe0i32)
                    as (u8);
            }

            if desired_settings.contains(DesiredSensorSettings::OSP_SEL) {
                debug!("OSP_SEL: true");
                let tph_sett_os_pres = tph_sett.os_temp.expect("OS TEMP");
                data = (data as (i32) & !0x1ci32 | tph_sett_os_pres as (i32) << 2i32 & 0x1ci32)
                    as (u8);
            }

            reg.push((BME680_CONF_T_P_MODE_ADDR, data));
        }

        /* Selecting humidity oversampling for the sensor */
        if desired_settings.contains(DesiredSensorSettings::OSH_SEL) {
            debug!("OSH_SEL: true");
            let tph_sett_os_hum = boundary_check::<I2C>(tph_sett.os_hum.map(|x| x as u8), 0, 5)?;
            let mut data = I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_OS_H_ADDR)?;
            data = (data as (i32) & !0x7i32 | tph_sett_os_hum as (i32) & 0x7i32) as (u8);
            reg.push((BME680_CONF_OS_H_ADDR, data));
        }

        /* Selecting the runGas and NB conversion settings for the sensor */
        if desired_settings
            .contains(DesiredSensorSettings::RUN_GAS_SEL | DesiredSensorSettings::NBCONV_SEL)
        {
            let mut data =
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_ODR_RUN_GAS_NBC_ADDR)?;

            if desired_settings.contains(DesiredSensorSettings::RUN_GAS_SEL) {
                debug!("RUN_GAS_SEL: true");
                let gas_sett_run_gas = boundary_check::<I2C>(gas_sett.run_gas, 0, 1)?;
                data = (data as (i32) & !0x10i32 | gas_sett_run_gas as (i32) << 4i32 & 0x10i32)
                    as (u8);
            }

            if desired_settings.contains(DesiredSensorSettings::NBCONV_SEL) {
                debug!("NBCONV_SEL: true");
                let gas_sett_nb_conv = boundary_check::<I2C>(Some(gas_sett.nb_conv), 0, 10)?;
                data = (data as (i32) & !0xfi32 | gas_sett_nb_conv as (i32) & 0xfi32) as (u8);
            }

            reg.push((BME680_CONF_ODR_RUN_GAS_NBC_ADDR, data));
        }

        self.bme680_set_regs(reg.as_slice())?;

        /* Restore previous intended power mode */
        self.power_mode = intended_power_mode;
        Ok(())
    }

    // TODO replace desired_settings with proper flags type see lib.rs
    pub fn get_sensor_settings(
        &mut self,
        desired_settings: DesiredSensorSettings,
    ) -> Result<SensorSettings, <I2C as Read>::Error, <I2C as Write>::Error> {
        let reg_addr: u8 = 0x70u8;
        let mut data_array: [u8; BME680_REG_BUFFER_LENGTH] = [0; BME680_REG_BUFFER_LENGTH];
        let mut sensor_settings: SensorSettings = Default::default();

        I2CUtil::read_bytes(&mut self.i2c, self.dev_id, reg_addr, &mut data_array)?;

        if desired_settings.contains(DesiredSensorSettings::GAS_MEAS_SEL) {
            sensor_settings.gas_sett = self.get_gas_config()?;
        }

        if desired_settings.contains(DesiredSensorSettings::FILTER_SEL) {
            sensor_settings.tph_sett.filter =
                Some(((data_array[5usize] as (i32) & 0x1ci32) >> 2i32) as (u8));
        }

        if desired_settings
            .contains(DesiredSensorSettings::OST_SEL | DesiredSensorSettings::OSP_SEL)
        {
            sensor_settings.tph_sett.os_temp = Some(OversamplingSetting::from(
                ((data_array[4usize] as (i32) & 0xe0i32) >> 5i32) as (u8),
            ));
            sensor_settings.tph_sett.os_pres = Some(OversamplingSetting::from(
                ((data_array[4usize] as (i32) & 0x1ci32) >> 2i32) as (u8),
            ));
        }

        if desired_settings.contains(DesiredSensorSettings::OSH_SEL) {
            sensor_settings.tph_sett.os_hum = Some(OversamplingSetting::from(
                (data_array[2usize] as (i32) & 0x7i32) as (u8),
            ));
        }

        if desired_settings.contains(DesiredSensorSettings::HCNTRL_SEL) {
            sensor_settings.gas_sett.heatr_ctrl =
                Some((data_array[0usize] as (i32) & 0x8i32) as (u8));
        }

        if desired_settings
            .contains(DesiredSensorSettings::RUN_GAS_SEL | DesiredSensorSettings::NBCONV_SEL)
        {
            sensor_settings.gas_sett.nb_conv = (data_array[1usize] as (i32) & 0xfi32) as (u8);
            sensor_settings.gas_sett.run_gas =
                Some(((data_array[1usize] as (i32) & 0x10i32) >> 4i32) as (u8));
        }

        Ok(sensor_settings)
    }

    pub fn set_sensor_mode(
        &mut self,
        target_power_mode: PowerMode,
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error> {
        let mut tmp_pow_mode: u8;
        let mut current_power_mode: PowerMode;

        /* Call repeatedly until in sleep */
        loop {
            tmp_pow_mode =
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_T_P_MODE_ADDR)?;

            /* Put to sleep before changing mode */
            current_power_mode = PowerMode::from(tmp_pow_mode & BME680_MODE_MSK);

            debug!("Current power mode: {:?}", current_power_mode);

            if current_power_mode != PowerMode::SleepMode {
                /* Set to sleep*/
                tmp_pow_mode = tmp_pow_mode & !BME680_MODE_MSK;
                debug!("Setting to sleep tmp_pow_mode: {}", tmp_pow_mode);
                self.bme680_set_regs(&[(BME680_CONF_T_P_MODE_ADDR, tmp_pow_mode)])?;
                self.delay.delay_ms(BME680_POLL_PERIOD_MS);
            } else {
                // TODO do while in Rust?
                break;
            }
        }

        /* Already in sleep */
        if target_power_mode != PowerMode::SleepMode {
            tmp_pow_mode = tmp_pow_mode & !BME680_MODE_MSK | target_power_mode.value();
            debug!("Already in sleep Target power mode: {}", tmp_pow_mode);
            self.bme680_set_regs(&[(BME680_CONF_T_P_MODE_ADDR, tmp_pow_mode)])?;
        }
        Ok(())
    }

    pub fn get_sensor_mode(
        &mut self,
    ) -> Result<PowerMode, <I2C as Read>::Error, <I2C as Write>::Error> {
        let regs = I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_CONF_T_P_MODE_ADDR)?;
        let mode = regs & BME680_MODE_MSK;
        Ok(PowerMode::from(mode))
    }

    pub fn bme680_set_profile_dur(&mut self, tph_sett: TphSett, duration: u16) {
        let os_to_meas_cycles: [u8; 6] = [0u8, 1u8, 2u8, 4u8, 8u8, 16u8];
        // TODO check if the following unwrap_ors do not change behaviour
        let mut meas_cycles = os_to_meas_cycles
            [tph_sett.os_temp.unwrap_or(OversamplingSetting::OSNone) as (usize)]
            as (u32);
        meas_cycles = meas_cycles.wrapping_add(
            os_to_meas_cycles[tph_sett.os_pres.unwrap_or(OversamplingSetting::OSNone) as (usize)]
                as (u32),
        );
        meas_cycles = meas_cycles.wrapping_add(
            os_to_meas_cycles[tph_sett.os_hum.unwrap_or(OversamplingSetting::OSNone) as (usize)]
                as (u32),
        );
        let mut tph_dur = meas_cycles.wrapping_mul(1963u32);
        tph_dur = tph_dur.wrapping_add(477u32.wrapping_mul(4u32));
        tph_dur = tph_dur.wrapping_add(477u32.wrapping_mul(5u32));
        tph_dur = tph_dur.wrapping_add(500u32);
        tph_dur = tph_dur.wrapping_div(1000u32);
        tph_dur = tph_dur.wrapping_add(1u32);
        self.gas_sett.heatr_dur = Some((duration as (i32) - tph_dur as (u16) as (i32)) as (u16));
    }

    pub fn get_profile_dur(
        &self,
        sensor_settings: &SensorSettings,
    ) -> Result<u16, <I2C as Read>::Error, <I2C as Write>::Error> {
        let os_to_meas_cycles: [u8; 6] = [0u8, 1u8, 2u8, 4u8, 8u8, 16u8];
        // TODO check if the following unwrap_ors do not change behaviour
        let mut meas_cycles = os_to_meas_cycles[sensor_settings
                                                    .tph_sett
                                                    .os_temp
                                                    .unwrap_or(OversamplingSetting::OSNone)
                                                    as (usize)]
            as (u32);
        meas_cycles = meas_cycles.wrapping_add(
            os_to_meas_cycles[sensor_settings
                                  .tph_sett
                                  .os_pres
                                  .unwrap_or(OversamplingSetting::OSNone)
                                  as (usize)] as (u32),
        );
        meas_cycles = meas_cycles.wrapping_add(
            os_to_meas_cycles[sensor_settings
                                  .tph_sett
                                  .os_hum
                                  .unwrap_or(OversamplingSetting::OSNone)
                                  as (usize)] as (u32),
        );
        let mut tph_dur = meas_cycles.wrapping_mul(1963u32);
        tph_dur = tph_dur.wrapping_add(477u32.wrapping_mul(4u32));
        tph_dur = tph_dur.wrapping_add(477u32.wrapping_mul(5u32));
        tph_dur = tph_dur.wrapping_add(500u32);
        tph_dur = tph_dur.wrapping_div(1000u32);
        tph_dur = tph_dur.wrapping_add(1u32);
        let mut duration = tph_dur as (u16);
        if sensor_settings.gas_sett.run_gas.unwrap_or(0) != 0 {
            duration = duration + sensor_settings.gas_sett.heatr_dur.expect("Heatrdur");
        }
        Ok(duration)
    }

    // TODO remove
    /// @returns (FieldData, IsNewFields)
    pub fn get_sensor_data(
        &mut self,
    ) -> Result<(FieldData, FieldDataState), <I2C as Read>::Error, <I2C as Write>::Error> {
        self.read_field_data()
    }

    fn get_calib_data<I2CX>(
        i2c: &mut I2CX,
        dev_id: u8,
    ) -> Result<CalibData, <I2CX as Read>::Error, <I2CX as Write>::Error>
    where
        I2CX: Read + Write,
    {
        let mut calib: CalibData = Default::default();

        // TODO figure out how to avoid allocating twice
        let mut coeff_vec = Vec::with_capacity(BME680_COEFF_SIZE);
        let mut coeff_array_1: [u8; BME680_COEFF_ADDR1_LEN] = [0; BME680_COEFF_ADDR1_LEN];
        let mut coeff_array_2: [u8; BME680_COEFF_ADDR2_LEN] = [0; BME680_COEFF_ADDR2_LEN];

        I2CUtil::read_bytes::<I2CX>(i2c, dev_id, BME680_COEFF_ADDR1, &mut coeff_array_1)?;

        I2CUtil::read_bytes::<I2CX>(i2c, dev_id, BME680_COEFF_ADDR2, &mut coeff_array_2)?;

        coeff_vec.extend_from_slice(&coeff_array_1);
        coeff_vec.extend_from_slice(&coeff_array_2);
        let coeff_array = coeff_vec.as_slice();

        calib.par_t1 = (coeff_array[34usize] as (u16) as (i32) << 8i32
            | coeff_array[33usize] as (u16) as (i32)) as (u16);
        calib.par_t2 = (coeff_array[2usize] as (u16) as (i32) << 8i32
            | coeff_array[1usize] as (u16) as (i32)) as (i16);
        calib.par_t3 = coeff_array[3usize] as (i8);
        calib.par_p1 = (coeff_array[6usize] as (u16) as (i32) << 8i32
            | coeff_array[5usize] as (u16) as (i32)) as (u16);
        calib.par_p2 = (coeff_array[8usize] as (u16) as (i32) << 8i32
            | coeff_array[7usize] as (u16) as (i32)) as (i16);
        calib.par_p3 = coeff_array[9usize] as (i8);
        calib.par_p4 = (coeff_array[12usize] as (u16) as (i32) << 8i32
            | coeff_array[11usize] as (u16) as (i32)) as (i16);
        calib.par_p5 = (coeff_array[14usize] as (u16) as (i32) << 8i32
            | coeff_array[13usize] as (u16) as (i32)) as (i16);
        calib.par_p6 = coeff_array[16usize] as (i8);
        calib.par_p7 = coeff_array[15usize] as (i8);
        calib.par_p8 = (coeff_array[20usize] as (u16) as (i32) << 8i32
            | coeff_array[19usize] as (u16) as (i32)) as (i16);
        calib.par_p9 = (coeff_array[22usize] as (u16) as (i32) << 8i32
            | coeff_array[21usize] as (u16) as (i32)) as (i16);
        calib.par_p10 = coeff_array[23usize];
        calib.par_h1 = (coeff_array[27usize] as (u16) as (i32) << 4i32
            | coeff_array[26usize] as (i32) & 0xfi32) as (u16);
        calib.par_h2 = (coeff_array[25usize] as (u16) as (i32) << 4i32
            | coeff_array[26usize] as (i32) >> 4i32) as (u16);
        calib.par_h3 = coeff_array[28usize] as (i8);
        calib.par_h4 = coeff_array[29usize] as (i8);
        calib.par_h5 = coeff_array[30usize] as (i8);
        calib.par_h6 = coeff_array[31usize];
        calib.par_h7 = coeff_array[32usize] as (i8);
        calib.par_gh1 = coeff_array[37usize] as (i8);
        calib.par_gh2 = (coeff_array[36usize] as (u16) as (i32) << 8i32
            | coeff_array[35usize] as (u16) as (i32)) as (i16);
        calib.par_gh3 = coeff_array[38usize] as (i8);

        calib.res_heat_range =
            (I2CUtil::read_byte::<I2CX>(i2c, dev_id, BME680_ADDR_RES_HEAT_RANGE_ADDR)? & 0x30) / 16;

        calib.res_heat_val =
            I2CUtil::read_byte::<I2CX>(i2c, dev_id, BME680_ADDR_RES_HEAT_VAL_ADDR)? as i8;

        calib.range_sw_err =
            (I2CUtil::read_byte::<I2CX>(i2c, dev_id, BME680_ADDR_RANGE_SW_ERR_ADDR)?
                & BME680_RSERROR_MSK) / 16;

        Ok(calib)
    }

    fn set_gas_config(
        &mut self,
        gas_sett: GasSett,
    ) -> Result<(), <I2C as Read>::Error, <I2C as Write>::Error> {
        if self.power_mode != PowerMode::ForcedMode {
            return Err(Bme680Error::DefinePwrMode);
        }

        // TODO check whether unwrap_or changes behaviour
        let reg: [(u8, u8); 2] = [
            (
                BME680_RES_HEAT0_ADDR,
                Calc::calc_heater_res(&self.calib, self.amb_temp, gas_sett.heatr_temp.unwrap_or(0)),
            ),
            (
                BME680_GAS_WAIT0_ADDR,
                Calc::calc_heater_dur(gas_sett.heatr_dur.unwrap_or(0)),
            ),
        ];

        self.gas_sett.nb_conv = 0;
        self.bme680_set_regs(&reg)
    }

    fn get_gas_config(&mut self) -> Result<GasSett, <I2C as Read>::Error, <I2C as Write>::Error> {
        // TODO move both GasSett fields to new struct
        let mut gas_sett: GasSett = Default::default();
        // TODO figure out if heat_temp and dur can be u8
        gas_sett.heatr_temp =
            Some(
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_ADDR_SENS_CONF_START)? as u16,
            );
        gas_sett.heatr_dur =
            Some(
                I2CUtil::read_byte(&mut self.i2c, self.dev_id, BME680_ADDR_GAS_CONF_START)? as u16,
            );
        Ok(gas_sett)
    }

    fn read_field_data(
        &mut self,
    ) -> Result<(FieldData, FieldDataState), <I2C as Read>::Error, <I2C as Write>::Error> {
        let mut buff: [u8; BME680_FIELD_LENGTH] = [0; BME680_FIELD_LENGTH];

        debug!("Buf {:?}, len: {}", buff, buff.len());
        let mut data: FieldData = Default::default();

        const TRIES: u8 = 10;
        for _ in 0..TRIES {
            I2CUtil::read_bytes(&mut self.i2c, self.dev_id, BME680_FIELD0_ADDR, &mut buff)?;

            debug!("Field data read {:?}, len: {}", buff, buff.len());

            data.status = buff[0] & BME680_NEW_DATA_MSK;
            data.gas_index = buff[0] & BME680_GAS_INDEX_MSK;;
            data.meas_index = buff[1];

            let adc_pres = (buff[2] as (u32)).wrapping_mul(4096)
                | (buff[3] as (u32)).wrapping_mul(16)
                | (buff[4] as (u32)).wrapping_div(16);
            let adc_temp = (buff[5] as (u32)).wrapping_mul(4096)
                | (buff[6] as (u32)).wrapping_mul(16)
                | (buff[7] as (u32)).wrapping_div(16);
            let adc_hum = ((buff[8] as (u32)).wrapping_mul(256) | buff[9] as (u32)) as (u16);
            let adc_gas_res = ((buff[13] as (u32)).wrapping_mul(4)
                | (buff[14] as (u32)).wrapping_div(64)) as (u16);
            let gas_range = buff[14] & BME680_GAS_RANGE_MSK;

            data.status = data.status | buff[14] & BME680_GASM_VALID_MSK;
            data.status = data.status | buff[14] & BME680_HEAT_STAB_MSK;

            if data.status & BME680_NEW_DATA_MSK != 0 {
                let (temp, t_fine) = Calc::calc_temperature(&self.calib, adc_temp);
                debug!(
                    "adc_temp: {} adc_pres: {} adc_hum: {} adc_gas_res: {}, t_fine: {}",
                    adc_temp, adc_pres, adc_hum, adc_gas_res, t_fine
                );
                data.temperature = temp;
                data.pressure = Calc::calc_pressure(&self.calib, t_fine, adc_pres);
                data.humidity = Calc::calc_humidity(&self.calib, t_fine, adc_hum);
                data.gas_resistance =
                    Calc::calc_gas_resistance(&self.calib, adc_gas_res, gas_range);
                return Ok((data, FieldDataState::NewData));
            }

            self.delay.delay_ms(BME680_POLL_PERIOD_MS);
        }
        Ok((data, FieldDataState::NoNewData))
    }
}
