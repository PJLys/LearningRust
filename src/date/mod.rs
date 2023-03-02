pub mod date {
    #[derive(Clone, Default)]
    pub struct Date {
        y: Year,
        m: Month,
        d: u8,
    }
    #[derive(Copy, Clone, Default)]
    pub struct Year {
        y: i16,
    }

    #[derive(Clone, Default)]
    pub struct Month {
        pub m: u8,
        nr_of_days: u8,
    }

    impl Year {
        pub fn is_leap_year(&self) -> bool {
            return (0 == self.y % 400) || (self.y % 100 != 0) && (self.y % 4) == 0;
        }

        pub fn new(y: i16) -> Result<Year, &'static str> {
            match y {
                0 => Err("The year 0 doesn't exist!"),
                _ => Ok(Year { y }),
            }
        }

        fn repr(&self) -> String {
            if self.y < 0 {
                return (-1 * self.y).to_string() + " BC";
            } else {
                return self.y.to_string() + " AD";
            }
        }
    }

    impl Month {
        pub fn new(m: u8, y: Year) -> Result<Month, &'static str> {
            let m_ran_30 = [4, 6, 9, 11];
            let m_ran_31 = [1, 3, 5, 7, 8, 10, 12];
            match (
                m_ran_30.contains(&m),
                m_ran_31.contains(&m),
                m == 2,
                y.is_leap_year(),
            ) {
                (true, false, _, _) => Ok(Month { m, nr_of_days: 30 }),
                (false, true, _, _) => Ok(Month { m, nr_of_days: 31 }),
                (false, false, true, false) => Ok(Month { m, nr_of_days: 28 }),
                (false, false, true, true) => Ok(Month { m, nr_of_days: 29 }),
                (_, _, _, _) => Err("Make sure the month is in the right range!"),
            }
        }

        fn repr(&self) -> &str {
            match self.m {
                1 => "January",
                2 => "February",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                12 => "December",
                _ => "None",
            }
        }

        pub fn get_nr_of_days(&self) -> &u8 {
            &self.nr_of_days
        }
    }

    impl Date {
        pub fn new(y: i16, m: u8, d: u8) -> Result<Date, &'static str> {
            let y = Year::new(y)?;
            let m: Month = Month::new(m, y)?;
            let d_ran = 0..m.nr_of_days + 1;
            match d_ran.contains(&d) {
                true => Ok(Date { y, m, d }),
                _ => Err("Day isn't in the right range"),
            }
        }

        pub fn repr(&self) -> String {
            self.m.repr().to_owned() + " " + &self.d.to_string().to_owned() + " " + &self.y.repr()
        }

        pub fn get_month(&self) -> &Month {
            &self.m
        }
    }
}
