macro_rules! tuplize {
    ($name:ident) => {
        $name!(I1);
        $name!(I1, I2);
        $name!(I1, I2, I3);
        $name!(I1, I2, I3, I4);
        $name!(I1, I2, I3, I4, I5);
        $name!(I1, I2, I3, I4, I5, I6);
        $name!(I1, I2, I3, I4, I5, I6, I7);
        $name!(I1, I2, I3, I4, I5, I6, I7, I8);
    };
}
