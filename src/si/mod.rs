//! [International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ)
//! implementations.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html

// #[macro_use]
// mod prefix;

uom_macros::system! {
    /// [International System of Quantities](http://jcgm.bipm.org/vim/en/1.6.html) (ISQ).
    name: ISQ;
    /// [International System of Units](http://jcgm.bipm.org/vim/en/1.16.html) (SI).
    units: SI;
    base {
        /// Length, one of the base quantities in the ISQ, denoted by the symbol L. The base unit
        /// for length is meter in the SI.
        length, L;
        /// Mass, one of the base quantities in the ISQ, denoted by the symbol M. The base unit
        /// for mass is kilogram in the SI.
        mass, M;
        /// Time, one of the base quantities in the ISQ, denoted by the symbol T. The base unit
        /// for time is second in the SI.
        time, T;
        /// Electric current, one of the base quantities in the ISQ, denoted by the symbol I. The
        /// base unit for electric current is ampere in the SI.
        electric_current, I;
        /// Thermodynamic temperature, one of the base quantities in the ISQ, denoted by the symbol
        /// Th (Θ). The base unit for thermodynamic temperature is kelvin in the SI.
        thermodynamic_temperature, Th;
        /// Amount of substance, one of the base quantities in the ISQ, denoted by the symbol N.
        /// The base unit for amount of substance is mole in the SI.
        amount_of_substance, N;
        /// Luminous intensity, one of the base quantities in the ISQ, denoted by the symbol J. The
        /// base unit for luminous intensity is candela in the SI.
        luminous_intensity, J;
    }
    quantities {
        mod acceleration::Acceleration;
        mod amount_of_substance::AmountOfSubstance;
        mod angle::Angle;
        mod angular_acceleration::AngularAcceleration;
        mod angular_velocity::AngularVelocity;
        mod area::Area;
        mod available_energy::AvailableEnergy;
        mod capacitance::Capacitance;
        mod electric_charge::ElectricCharge;
        mod electric_current::ElectricCurrent;
        mod electric_potential::ElectricPotential;
        mod electrical_conductance::ElectricalConductance;
        mod electrical_resistance::ElectricalResistance;
        mod energy::Energy;
        mod force::Force;
        mod frequency::Frequency;
        mod inductance::Inductance;
        mod jerk::Jerk;
        mod length::Length;
        mod luminance::Luminance;
        mod luminous_intensity::LuminousIntensity;
        mod magnetic_flux::MagneticFlux;
        mod magnetic_flux_density::MagneticFluxDensity;
        mod mass::Mass;
        mod mass_density::MassDensity;
        mod mass_rate::MassRate;
        mod momentum::Momentum;
        mod power::Power;
        mod pressure::Pressure;
        mod ratio::Ratio;
        mod temperature_interval::TemperatureInterval;
        mod thermodynamic_temperature::ThermodynamicTemperature;
        mod time::Time;
        mod torque::Torque;
        mod velocity::Velocity;
        mod volume::Volume;
        mod volume_rate::VolumeRate;
    }
}

///// [`Quantity`](struct.Quantity.html) type aliases using the default base units and parameterized
///// on the underlying storage type.
//pub mod quantities {
//    ISQ!(si);
//}

//storage_types! {
//    /// [`Quantity`](struct.Quantity.html) type aliases using the default base units.
//    pub types: All;

//    ISQ!(si, V);
//}

///// Primitive traits and types representing basic properties of types specific to the SI.
//pub mod marker {
//    use si::{change_base, Dimension, Quantity, Units};
//    use Kind;

//    /// AngleKind is a `Kind` for separating angular quantities from their indentically dimensioned
//    /// non-angular quantity counterparts. Conversions to and from `AngleKind` quantities is
//    /// supported through implementations of the `From` trait.
//    ///
//    #[cfg_attr(feature = "f32", doc = " ```rust")]
//    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
//    /// # use uom::si::f32::*;
//    /// # use uom::si::angle::radian;
//    /// let a: Angle = Angle::new::<radian>(1.0);
//    /// let r: Ratio = a.into();
//    /// ```
//    pub trait AngleKind: ::Kind {}

//    /// Kind of thermodynamic temperature.
//    pub trait TemperatureKind:
//        ::marker::Mul
//        + ::marker::MulAssign
//        + ::marker::Div
//        + ::marker::DivAssign
//        + ::marker::Rem
//        + ::marker::RemAssign
//    {
//    }

//    /// `impl_from` generates generic inter-Kind implementations of `From`.
//    #[macro_export]
//    macro_rules! impl_from {
//        ($a:ident, $b:ident) => {
//            impl<L, M, T, I, Th, N, J, Ul, Ur, V>
//                From<
//                    Quantity<
//                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
//                        Ur,
//                        V,
//                    >,
//                >
//                for Quantity<
//                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
//                    Ul,
//                    V,
//                >
//            where
//                Ul: Units<V> + ?Sized,
//                Ur: Units<V> + ?Sized,
//                V: ::num_traits::Num + ::Conversion<V>,
//            {
//                fn from(
//                    val: Quantity<
//                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
//                        Ur,
//                        V,
//                    >,
//                ) -> Quantity<
//                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
//                    Ul,
//                    V,
//                > {
//                    Self {
//                        dimension: ::lib::marker::PhantomData,
//                        units: ::lib::marker::PhantomData,
//                        value: change_base::<
//                            Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
//                            Ul,
//                            Ur,
//                            V,
//                        >(&val.value),
//                    }
//                }
//            }
//        };
//    }

//    impl_from!(AngleKind, Kind);
//    impl_from!(Kind, AngleKind);
//}
