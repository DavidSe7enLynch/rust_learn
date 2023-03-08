use log::info;

#[macro_export]
macro_rules! hrrVec {
    ($($x:expr);*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}

pub fn hrr_vec() {
    let hrrvec = hrrVec!(1; 2; 3);
    info!("hrrvec = {:?}", hrrvec);
}