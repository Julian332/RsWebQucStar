macro_rules! vec_as_params {
     ( $( $x:ident ),* ) => {
        {
            let mut data: [f32; SIZE] = [0.0; SIZE];
            let mut index = 0;
            $(
                #[allow(unused_assignments)]
                {
                    data[index] = $x;
                    index = index + 1;
                }
            )*
            MyVec { data }
        }
    };
}

fn main() {}
