use core::cell::Cell;
use core::ops::Deref;
use critical_section::Mutex;
use embedded_hal_0_2::can::{Frame, Id, StandardId};
use mcp2515::frame::CanFrame;
use strum_macros::FromRepr;
use paste::paste;

#[non_exhaustive]
pub enum DataWidth {
    U8,
    U16,
    I16,
}

impl DataWidth {
    pub fn num_bytes(&self) -> usize {
        match self {
            DataWidth::U8 => 1,
            DataWidth::U16 => 2,
            DataWidth::I16 => 2,
        }
    }
}

pub struct GaugeData {
    pub id: u16,
    pub width: DataWidth,
    pub value: Mutex<Cell<u32>>,
}

impl GaugeData {
    pub const fn new(id: u16, width: DataWidth, initial_value: u32) -> Self {
        GaugeData {
            id,
            width,
            value: Mutex::new(Cell::new(initial_value)),
        }
    }

    pub fn width(&self) -> usize {
        self.width.num_bytes()
    }

    pub fn to_frame(&self) -> Option<CanFrame> {
        let data = &self.get().to_le_bytes()[..self.width()];
        CanFrame::new(Id::Standard(StandardId::new(self.id.into())?), data)
    }

    pub fn get(&self) -> u32 {
        critical_section::with(|cs| {
            let cell = self.value.borrow(cs).get();
            cell
        })
    }

    pub fn set(&self, value: u32) {
        critical_section::with(|cs| {
            self.value.borrow(cs).set(value);
        })
    }


    pub fn set_from_bytes(&self, data: &[u8]) {
        let mut bytes = [0; 4];
        bytes[0..data.len()].copy_from_slice(data);

        self.set(u32::from_le_bytes(bytes));
    }

    pub fn set_from_frame(&self, frame: CanFrame) {
        let data = &frame.data()[..frame.dlc()];

        self.set_from_bytes(&data);
    }
    pub fn prim_id(&self) -> u8 {
        let result = self.id.try_into().unwrap();
        result
    }
}

macro_rules! gauges {
    ($($name:expr, $id:expr, $w:expr),+) => {
        $(
        paste! {
            pub static $name: GaugeData = GaugeData::new($id, $w, 0);
        }
        )+
    };
}

#[repr(u16)]
#[derive(Debug, FromRepr)]
pub enum Gauge {
    StaTime = 0x20,
    StaStatus1 = 0x21,
    StaEng = 0x22,
    DWELL = 0x23,
    MAP = 0x24,
    IAT = 0x25,
    CLNT = 0x26,
    BatCorrect = 0x27,
    BatVol = 0x28,
    AfrPri = 0x29,
    EgoCorrect = 0x2A,
    IatCorrect = 0x2B,
    WueCorrect = 0x2C,
    RPM = 0x2D,
    AccelEnrich = 0x2E,
    GammeE = 0x2F,
    VE = 0x30,
    AfrTarget = 0x31,
    PulseWidth1 = 0x32,
    TpsDot = 0x33,
    CurSparkAdvance = 0x34,
    TPS = 0x35,
    LoopPs = 0x36,
    FreeMem = 0x37,
    BoostTarget = 0x38,
    BoostPwm = 0x39,
    StaSpark = 0x3A,
    RpmDot = 0x3B,
    EthanolPercent = 0x3C,
    FlexCorrect = 0x3D,
    FlexIgnCorrect = 0x3E,
    IdleLoad = 0x3F,
    TestOutputs = 0x40,
    AfrSec = 0x41,
    BARO = 0x42,
    TpsAdc = 0x43,
    NextError = 0x44,
    StaLaunchCorrect = 0x45,
    PulseWidth2 = 0x46,
    PulseWidth3 = 0x47,
    PulseWidth4 = 0x48,
    StaStatus2 = 0x49,
    EngProtectSta = 0x4A,
    FuelLoad = 0x4B,
    IgnLoad = 0x4C,
    InjAngle = 0x4D,
    IdleDuty = 0x4E,
    ClIdleTarget = 0x4F,
    MapDot = 0x50,
    VvtAngle = 0x51,
    VvtTargetAngle = 0x52,
    VvtDuty = 0x53,
    FlexBoostCorrect = 0x54,
    BaroCorrection = 0x55,
    ASE = 0x56,
    VSS = 0x57,
    GEAR = 0x58,
    FuelPres = 0x59,
    OilPres = 0x5A,
    WmiPw = 0x5B,
    StaStatus4 = 0x5C,
    VvtAngle2 = 0x5D,
    VvtTargetAngle2 = 0x5E,
    VvtDuty2 = 0x5F,
    StatusOutSta = 0x60,
    FlexFuelTemp = 0x61,
    FuelTempCorrect = 0x62,
    VE1 = 0x63,
    VE2 = 0x64,
    ADVANCE1 = 0x66,
    ADVANCE2 = 0x67,
    NitroSta = 0x68,
    SdSta = 0x69,
    Masteralive = 0x70
}

impl Gauge {
    fn raw_gauge(&self) -> &'static GaugeData {
        match self {
            Gauge::StaTime => &STA_TIME,
            Gauge::StaStatus1 => &STA_STATUS1,
            Gauge::StaEng => &STA_ENG,
            Gauge::DWELL => &DWELL,
            Gauge::MAP => &MAP,
            Gauge::IAT => &IAT,
            Gauge::CLNT => &CLNT,
            Gauge::BatCorrect => &BAT_CORRECT,
            Gauge::BatVol => &BAT_VOL,
            Gauge::AfrPri => &AFR_PRI,
            Gauge::EgoCorrect => &EGO_CORRECT,
            Gauge::IatCorrect => &IAT_CORRECT,
            Gauge::WueCorrect => &WUE_CORRECT,
            Gauge::RPM => &RPM,
            Gauge::AccelEnrich => &ACCEL_ENRICH,
            Gauge::GammeE => &GAMME_E,
            Gauge::VE => &VE,
            Gauge::AfrTarget => &AFR_TARGET,
            Gauge::PulseWidth1 => &PULSE_WIDTH1,
            Gauge::TpsDot => &TPS_DOT,
            Gauge::CurSparkAdvance => &CUR_SPARK_ADVANCE,
            Gauge::TPS => &TPS,
            Gauge::LoopPs => &LOOP_PS,
            Gauge::FreeMem => &FREE_MEM,
            Gauge::BoostTarget => &BOOST_TARGET,
            Gauge::BoostPwm => &BOOST_PWM,
            Gauge::StaSpark => &STA_SPARK,
            Gauge::RpmDot => &RPM_DOT,
            Gauge::EthanolPercent => &ETHANOL_PERCENT,
            Gauge::FlexCorrect => &FLEX_CORRECT,
            Gauge::FlexIgnCorrect => &FLEX_IGN_CORRECT,
            Gauge::IdleLoad => &IDLE_LOAD,
            Gauge::TestOutputs => &TEST_OUTPUTS,
            Gauge::AfrSec => &AFR_SEC,
            Gauge::BARO => &BARO,
            Gauge::TpsAdc => &TPS_ADC,
            Gauge::NextError => &NEXT_ERROR,
            Gauge::StaLaunchCorrect => &STA_LAUNCH_CORRECT,
            Gauge::PulseWidth2 => &PULSE_WIDTH2,
            Gauge::PulseWidth3 => &PULSE_WIDTH3,
            Gauge::PulseWidth4 => &PULSE_WIDTH4,
            Gauge::StaStatus2 => &STA_STATUS2,
            Gauge::EngProtectSta => &ENG_PROTECT_STA,
            Gauge::FuelLoad => &FUEL_LOAD,
            Gauge::IgnLoad => &IGN_LOAD,
            Gauge::InjAngle => &INJ_ANGLE,
            Gauge::IdleDuty => &IDLE_DUTY,
            Gauge::ClIdleTarget => &CL_IDLE_TARGET,
            Gauge::MapDot => &MAP_DOT,
            Gauge::VvtAngle => &VVT_ANGLE,
            Gauge::VvtTargetAngle => &VVT_TARGET_ANGLE,
            Gauge::VvtDuty => &VVT_DUTY,
            Gauge::FlexBoostCorrect => &FLEX_BOOST_CORRECT,
            Gauge::BaroCorrection => &BARO_CORRECTION,
            Gauge::ASE => &ASE,
            Gauge::VSS => &VSS,
            Gauge::GEAR => &GEAR,
            Gauge::FuelPres => &FUEL_PRES,
            Gauge::OilPres => &OIL_PRES,
            Gauge::WmiPw => &WMI_PW,
            Gauge::StaStatus4 => &STA_STATUS4,
            Gauge::VvtAngle2 => &VVT_ANGLE2,
            Gauge::VvtTargetAngle2 => &VVT_TARGET_ANGLE2,
            Gauge::VvtDuty2 => &VVT_DUTY2,
            Gauge::StatusOutSta => &STATUS_OUT_STA,
            Gauge::FlexFuelTemp => &FLEX_FUEL_TEMP,
            Gauge::FuelTempCorrect => &FUEL_TEMP_CORRECT,
            Gauge::VE1 => &VE1,
            Gauge::VE2 => &VE2,
            Gauge::ADVANCE1 => &ADVANCE1,
            Gauge::ADVANCE2 => &ADVANCE2,
            Gauge::NitroSta => &NITRO_STA,
            Gauge::SdSta => &SD_STA,
            Gauge::Masteralive => &MASTERALIVE,
        }
    }
}

impl Deref for Gauge {
    type Target = GaugeData;

    fn deref(&self) -> &'static Self::Target {
        self.raw_gauge()
    }
}

gauges! {
    STA_TIME, 0x20, DataWidth::U8,
    STA_STATUS1, 0x21, DataWidth::U8,
    STA_ENG, 0x22, DataWidth::U8,
    DWELL, 0x23, DataWidth::U8,
    MAP, 0x24, DataWidth::U16,
    IAT, 0x25, DataWidth::U8,
    CLNT, 0x26, DataWidth::U8,
    BAT_CORRECT, 0x27, DataWidth::U8,
    BAT_VOL, 0x28, DataWidth::U8,
    AFR_PRI, 0x29, DataWidth::U8,
    EGO_CORRECT, 0x2A, DataWidth::U8,
    IAT_CORRECT, 0x2B, DataWidth::U8,
    WUE_CORRECT, 0x2C, DataWidth::U8,
    RPM, 0x2D, DataWidth::U16,
    ACCEL_ENRICH, 0x2E, DataWidth::U8,
    GAMME_E, 0x2F, DataWidth::U8,
    VE, 0x30, DataWidth::U8,
    AFR_TARGET, 0x31, DataWidth::U8,
    PULSE_WIDTH1, 0x32, DataWidth::U16,
    TPS_DOT, 0x33, DataWidth::U8,
    CUR_SPARK_ADVANCE, 0x34, DataWidth::U8,
    TPS, 0x35, DataWidth::U8,
    LOOP_PS, 0x36, DataWidth::U16,
    FREE_MEM, 0x37, DataWidth::U16,
    BOOST_TARGET, 0x38, DataWidth::U8,
    BOOST_PWM, 0x39, DataWidth::U8,
    STA_SPARK, 0x3A, DataWidth::U8,
    RPM_DOT, 0x3B, DataWidth::I16,
    ETHANOL_PERCENT, 0x3C, DataWidth::U8,
    FLEX_CORRECT, 0x3D, DataWidth::U8,
    FLEX_IGN_CORRECT, 0x3E, DataWidth::U8,
    IDLE_LOAD, 0x3F, DataWidth::U8,
    TEST_OUTPUTS, 0x40, DataWidth::U8,
    AFR_SEC, 0x41, DataWidth::U8,
    BARO, 0x42, DataWidth::U8,
    TPS_ADC, 0x43, DataWidth::U8,
    NEXT_ERROR, 0x44, DataWidth::U8,
    STA_LAUNCH_CORRECT, 0x45, DataWidth::U8,
    PULSE_WIDTH2, 0x46, DataWidth::U8,
    PULSE_WIDTH3, 0x47, DataWidth::U8,
    PULSE_WIDTH4, 0x48, DataWidth::U8,
    STA_STATUS2, 0x49, DataWidth::U8,
    ENG_PROTECT_STA, 0x4A, DataWidth::U8,
    FUEL_LOAD, 0x4B, DataWidth::U16,
    IGN_LOAD, 0x4C, DataWidth::U16,
    INJ_ANGLE, 0x4D, DataWidth::U16,
    IDLE_DUTY, 0x4E, DataWidth::U8,
    CL_IDLE_TARGET, 0x4F, DataWidth::U8,
    MAP_DOT, 0x50, DataWidth::U8,
    VVT_ANGLE, 0x51, DataWidth::U8,
    VVT_TARGET_ANGLE, 0x52, DataWidth::U8,
    VVT_DUTY, 0x53, DataWidth::U8,
    FLEX_BOOST_CORRECT, 0x54, DataWidth::U16,
    BARO_CORRECTION, 0x55, DataWidth::U8,
    ASE, 0x56, DataWidth::U8,
    VSS, 0x57, DataWidth::U16,
    GEAR, 0x58, DataWidth::U8,
    FUEL_PRES, 0x59, DataWidth::U8,
    OIL_PRES, 0x5A, DataWidth::U8,
    WMI_PW, 0x5B, DataWidth::U8,
    STA_STATUS4, 0x5C, DataWidth::U8,
    VVT_ANGLE2, 0x5D, DataWidth::U8,
    VVT_TARGET_ANGLE2, 0x5E, DataWidth::U8,
    VVT_DUTY2, 0x5F, DataWidth::U8,
    STATUS_OUT_STA, 0x60, DataWidth::U8,
    FLEX_FUEL_TEMP, 0x61, DataWidth::U8,
    FUEL_TEMP_CORRECT, 0x62, DataWidth::U8,
    VE1, 0x63, DataWidth::U8,
    VE2, 0x64, DataWidth::U8,
    ADVANCE1, 0x66, DataWidth::U8,
    ADVANCE2, 0x67, DataWidth::U8,
    NITRO_STA, 0x68, DataWidth::U8,
    SD_STA, 0x69, DataWidth::U8,
    MASTERALIVE, 0x70, DataWidth::U8
}
