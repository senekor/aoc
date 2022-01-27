use num::BigUint;

mod pt {
    pub fn generate_periodic_table() {
        let input = include_str!("../input/periodic_table.txt");
        let mut lines = input.split("\n");
        let elements: Vec<&str> = lines.next().unwrap().split(" ").collect();
        lines.next();
        let strings: Vec<&str> = lines.collect();
        let input = include_str!("../input/to_next.txt");
        let nexts: Vec<&str> = input.split("\n").collect();
        let mut constants = String::from("");
        let mut str_to_nr_match_stmt = String::from("");
        let mut nr_to_len_match_stmt = String::from("");
        let mut nr_to_next_match_stmt = String::from("");
        for i_rev in 0..92 {
            let i = 91 - i_rev;
            let name = elements[i].to_ascii_uppercase();
            constants = format!(
                "{}pub const {n:_<2}: &str = \"{s}\";\n",
                constants,
                n = name,
                s = strings[i]
            );
            constants = format!(
                "{}pub const {n:_<2}_N: &[usize] = &[{next}];\n",
                constants,
                n = name,
                next = nexts[i]
            );
            str_to_nr_match_stmt = format!(
                "{}pt_const::{n:_<2} => {idx},\n",
                str_to_nr_match_stmt,
                n = name,
                idx = i_rev,
            );
            nr_to_len_match_stmt = format!(
                "{}{idx} => pt_const::{n:_<2}.len(),\n",
                nr_to_len_match_stmt,
                idx = i_rev,
                n = name,
            );
            nr_to_next_match_stmt = format!(
                "{}{idx} => pt_const::{n:_<2}_N,\n",
                nr_to_next_match_stmt,
                idx = i_rev,
                n = name,
            );
        }
        if let Err(e) = std::fs::write("output/periodic_table/constants.txt", constants) {
            println!("failed to write constants: {}", e)
        }
        if let Err(e) = std::fs::write("output/periodic_table/str_to_nr.txt", str_to_nr_match_stmt)
        {
            println!("failed to write str_to_nr: {}", e)
        }
        if let Err(e) = std::fs::write("output/periodic_table/nr_to_len.txt", nr_to_len_match_stmt)
        {
            println!("failed to write nr_to_len: {}", e)
        }
        if let Err(e) = std::fs::write(
            "output/periodic_table/nr_to_next.txt",
            nr_to_next_match_stmt,
        ) {
            println!("failed to write nr_to_next: {}", e)
        }
    }
    mod pt_const {
        pub const H_: &str = "22";
        pub const H__N: &[usize] = &[0];
        pub const HE: &str = "13112221133211322112211213322112";
        pub const HE_N: &[usize] = &[71, 90, 0, 19, 2];
        pub const LI: &str = "312211322212221121123222112";
        pub const LI_N: &[usize] = &[1];
        pub const BE: &str = "111312211312113221133211322112211213322112";
        pub const BE_N: &[usize] = &[31, 19, 2];
        pub const B_: &str = "1321132122211322212221121123222112";
        pub const B__N: &[usize] = &[3];
        pub const C_: &str = "3113112211322112211213322112";
        pub const C__N: &[usize] = &[4];
        pub const N_: &str = "111312212221121123222112";
        pub const N__N: &[usize] = &[5];
        pub const O_: &str = "132112211213322112";
        pub const O__N: &[usize] = &[6];
        pub const F_: &str = "31121123222112";
        pub const F__N: &[usize] = &[7];
        pub const NE: &str = "111213322112";
        pub const NE_N: &[usize] = &[8];
        pub const NA: &str = "123222112";
        pub const NA_N: &[usize] = &[9];
        pub const MG: &str = "3113322112";
        pub const MG_N: &[usize] = &[60, 10];
        pub const AL: &str = "1113222112";
        pub const AL_N: &[usize] = &[11];
        pub const SI: &str = "1322112";
        pub const SI_N: &[usize] = &[12];
        pub const P_: &str = "311311222112";
        pub const P__N: &[usize] = &[66, 13];
        pub const S_: &str = "1113122112";
        pub const S__N: &[usize] = &[14];
        pub const CL: &str = "132112";
        pub const CL_N: &[usize] = &[15];
        pub const AR: &str = "3112";
        pub const AR_N: &[usize] = &[16];
        pub const K_: &str = "1112";
        pub const K__N: &[usize] = &[17];
        pub const CA: &str = "12";
        pub const CA_N: &[usize] = &[18];
        pub const SC: &str = "3113112221133112";
        pub const SC_N: &[usize] = &[66, 90, 0, 19, 26];
        pub const TI: &str = "11131221131112";
        pub const TI_N: &[usize] = &[20];
        pub const V_: &str = "13211312";
        pub const V__N: &[usize] = &[21];
        pub const CR: &str = "31132";
        pub const CR_N: &[usize] = &[22];
        pub const MN: &str = "111311222112";
        pub const MN_N: &[usize] = &[23, 13];
        pub const FE: &str = "13122112";
        pub const FE_N: &[usize] = &[24];
        pub const CO: &str = "32112";
        pub const CO_N: &[usize] = &[25];
        pub const NI: &str = "11133112";
        pub const NI_N: &[usize] = &[29, 26];
        pub const CU: &str = "131112";
        pub const CU_N: &[usize] = &[27];
        pub const ZN: &str = "312";
        pub const ZN_N: &[usize] = &[28];
        pub const GA: &str = "13221133122211332";
        pub const GA_N: &[usize] = &[62, 19, 88, 0, 19, 29];
        pub const GE: &str = "31131122211311122113222";
        pub const GE_N: &[usize] = &[66, 30];
        pub const AS: &str = "11131221131211322113322112";
        pub const AS_N: &[usize] = &[31, 10];
        pub const SE: &str = "13211321222113222112";
        pub const SE_N: &[usize] = &[32];
        pub const BR: &str = "3113112211322112";
        pub const BR_N: &[usize] = &[33];
        pub const KR: &str = "11131221222112";
        pub const KR_N: &[usize] = &[34];
        pub const RB: &str = "1321122112";
        pub const RB_N: &[usize] = &[35];
        pub const SR: &str = "3112112";
        pub const SR_N: &[usize] = &[36];
        pub const Y_: &str = "1112133";
        pub const Y__N: &[usize] = &[37, 91];
        pub const ZR: &str = "12322211331222113112211";
        pub const ZR_N: &[usize] = &[38, 0, 19, 42];
        pub const NB: &str = "1113122113322113111221131221";
        pub const NB_N: &[usize] = &[67, 39];
        pub const MO: &str = "13211322211312113211";
        pub const MO_N: &[usize] = &[40];
        pub const TC: &str = "311322113212221";
        pub const TC_N: &[usize] = &[41];
        pub const RU: &str = "132211331222113112211";
        pub const RU_N: &[usize] = &[62, 19, 42];
        pub const RH: &str = "311311222113111221131221";
        pub const RH_N: &[usize] = &[66, 43];
        pub const PD: &str = "111312211312113211";
        pub const PD_N: &[usize] = &[44];
        pub const AG: &str = "132113212221";
        pub const AG_N: &[usize] = &[45];
        pub const CD: &str = "3113112211";
        pub const CD_N: &[usize] = &[46];
        pub const IN: &str = "11131221";
        pub const IN_N: &[usize] = &[47];
        pub const SN: &str = "13211";
        pub const SN_N: &[usize] = &[48];
        pub const SB: &str = "3112221";
        pub const SB_N: &[usize] = &[60, 49];
        pub const TE: &str = "1322113312211";
        pub const TE_N: &[usize] = &[62, 19, 50];
        pub const I_: &str = "311311222113111221";
        pub const I__N: &[usize] = &[66, 51];
        pub const XE: &str = "11131221131211";
        pub const XE_N: &[usize] = &[52];
        pub const CS: &str = "13211321";
        pub const CS_N: &[usize] = &[53];
        pub const BA: &str = "311311";
        pub const BA_N: &[usize] = &[54];
        pub const LA: &str = "11131";
        pub const LA_N: &[usize] = &[55];
        pub const CE: &str = "1321133112";
        pub const CE_N: &[usize] = &[56, 0, 19, 26];
        pub const PR: &str = "31131112";
        pub const PR_N: &[usize] = &[57];
        pub const ND: &str = "111312";
        pub const ND_N: &[usize] = &[58];
        pub const PM: &str = "132";
        pub const PM_N: &[usize] = &[59];
        pub const SM: &str = "311332";
        pub const SM_N: &[usize] = &[60, 19, 29];
        pub const EU: &str = "1113222";
        pub const EU_N: &[usize] = &[61];
        pub const GD: &str = "13221133112";
        pub const GD_N: &[usize] = &[62, 19, 26];
        pub const TB: &str = "3113112221131112";
        pub const TB_N: &[usize] = &[66, 63];
        pub const DY: &str = "111312211312";
        pub const DY_N: &[usize] = &[64];
        pub const HO: &str = "1321132";
        pub const HO_N: &[usize] = &[65];
        pub const ER: &str = "311311222";
        pub const ER_N: &[usize] = &[66, 60];
        pub const TM: &str = "11131221133112";
        pub const TM_N: &[usize] = &[67, 19, 26];
        pub const YB: &str = "1321131112";
        pub const YB_N: &[usize] = &[68];
        pub const LU: &str = "311312";
        pub const LU_N: &[usize] = &[69];
        pub const HF: &str = "11132";
        pub const HF_N: &[usize] = &[70];
        pub const TA: &str = "13112221133211322112211213322113";
        pub const TA_N: &[usize] = &[71, 90, 0, 19, 73];
        pub const W_: &str = "312211322212221121123222113";
        pub const W__N: &[usize] = &[72];
        pub const RE: &str = "111312211312113221133211322112211213322113";
        pub const RE_N: &[usize] = &[31, 19, 73];
        pub const OS: &str = "1321132122211322212221121123222113";
        pub const OS_N: &[usize] = &[74];
        pub const IR: &str = "3113112211322112211213322113";
        pub const IR_N: &[usize] = &[75];
        pub const PT: &str = "111312212221121123222113";
        pub const PT_N: &[usize] = &[76];
        pub const AU: &str = "132112211213322113";
        pub const AU_N: &[usize] = &[77];
        pub const HG: &str = "31121123222113";
        pub const HG_N: &[usize] = &[78];
        pub const TL: &str = "111213322113";
        pub const TL_N: &[usize] = &[79];
        pub const PB: &str = "123222113";
        pub const PB_N: &[usize] = &[80];
        pub const BI: &str = "3113322113";
        pub const BI_N: &[usize] = &[60, 81];
        pub const PO: &str = "1113222113";
        pub const PO_N: &[usize] = &[82];
        pub const AT: &str = "1322113";
        pub const AT_N: &[usize] = &[83];
        pub const RN: &str = "311311222113";
        pub const RN_N: &[usize] = &[66, 84];
        pub const FR: &str = "1113122113";
        pub const FR_N: &[usize] = &[85];
        pub const RA: &str = "132113";
        pub const RA_N: &[usize] = &[86];
        pub const AC: &str = "3113";
        pub const AC_N: &[usize] = &[87];
        pub const TH: &str = "1113";
        pub const TH_N: &[usize] = &[88];
        pub const PA: &str = "13";
        pub const PA_N: &[usize] = &[89];
        pub const U_: &str = "3";
        pub const U__N: &[usize] = &[90];

        // custom transuranic elements. last, variable digit in string omitted
        pub const NP: &str = "1311222113321132211221121332211";
        pub const NP_N: &[usize] = &[71, 90, 0, 19, 93];
        pub const PU: &str = "31221132221222112112322211";
        pub const PU_N: &[usize] = &[92];
    }
    pub fn str_to_nr(elem: &str) -> usize {
        match elem {
            pt_const::H_ => 0,
            pt_const::HE => 1,
            pt_const::LI => 2,
            pt_const::BE => 3,
            pt_const::B_ => 4,
            pt_const::C_ => 5,
            pt_const::N_ => 6,
            pt_const::O_ => 7,
            pt_const::F_ => 8,
            pt_const::NE => 9,
            pt_const::NA => 10,
            pt_const::MG => 11,
            pt_const::AL => 12,
            pt_const::SI => 13,
            pt_const::P_ => 14,
            pt_const::S_ => 15,
            pt_const::CL => 16,
            pt_const::AR => 17,
            pt_const::K_ => 18,
            pt_const::CA => 19,
            pt_const::SC => 20,
            pt_const::TI => 21,
            pt_const::V_ => 22,
            pt_const::CR => 23,
            pt_const::MN => 24,
            pt_const::FE => 25,
            pt_const::CO => 26,
            pt_const::NI => 27,
            pt_const::CU => 28,
            pt_const::ZN => 29,
            pt_const::GA => 30,
            pt_const::GE => 31,
            pt_const::AS => 32,
            pt_const::SE => 33,
            pt_const::BR => 34,
            pt_const::KR => 35,
            pt_const::RB => 36,
            pt_const::SR => 37,
            pt_const::Y_ => 38,
            pt_const::ZR => 39,
            pt_const::NB => 40,
            pt_const::MO => 41,
            pt_const::TC => 42,
            pt_const::RU => 43,
            pt_const::RH => 44,
            pt_const::PD => 45,
            pt_const::AG => 46,
            pt_const::CD => 47,
            pt_const::IN => 48,
            pt_const::SN => 49,
            pt_const::SB => 50,
            pt_const::TE => 51,
            pt_const::I_ => 52,
            pt_const::XE => 53,
            pt_const::CS => 54,
            pt_const::BA => 55,
            pt_const::LA => 56,
            pt_const::CE => 57,
            pt_const::PR => 58,
            pt_const::ND => 59,
            pt_const::PM => 60,
            pt_const::SM => 61,
            pt_const::EU => 62,
            pt_const::GD => 63,
            pt_const::TB => 64,
            pt_const::DY => 65,
            pt_const::HO => 66,
            pt_const::ER => 67,
            pt_const::TM => 68,
            pt_const::YB => 69,
            pt_const::LU => 70,
            pt_const::HF => 71,
            pt_const::TA => 72,
            pt_const::W_ => 73,
            pt_const::RE => 74,
            pt_const::OS => 75,
            pt_const::IR => 76,
            pt_const::PT => 77,
            pt_const::AU => 78,
            pt_const::HG => 79,
            pt_const::TL => 80,
            pt_const::PB => 81,
            pt_const::BI => 82,
            pt_const::PO => 83,
            pt_const::AT => 84,
            pt_const::RN => 85,
            pt_const::FR => 86,
            pt_const::RA => 87,
            pt_const::AC => 88,
            pt_const::TH => 89,
            pt_const::PA => 90,
            pt_const::U_ => 91,
            // custom cases for transuranic elements
            _ => match elem.get(0..elem.len() - 1).unwrap() {
                pt_const::NP => 92,
                pt_const::PU => 93,
                _ => panic!("element not recognized: {}", elem),
            },
        }
    }
    pub fn nr_to_len(elem_nr: usize) -> usize {
        match elem_nr {
            0 => pt_const::H_.len(),
            1 => pt_const::HE.len(),
            2 => pt_const::LI.len(),
            3 => pt_const::BE.len(),
            4 => pt_const::B_.len(),
            5 => pt_const::C_.len(),
            6 => pt_const::N_.len(),
            7 => pt_const::O_.len(),
            8 => pt_const::F_.len(),
            9 => pt_const::NE.len(),
            10 => pt_const::NA.len(),
            11 => pt_const::MG.len(),
            12 => pt_const::AL.len(),
            13 => pt_const::SI.len(),
            14 => pt_const::P_.len(),
            15 => pt_const::S_.len(),
            16 => pt_const::CL.len(),
            17 => pt_const::AR.len(),
            18 => pt_const::K_.len(),
            19 => pt_const::CA.len(),
            20 => pt_const::SC.len(),
            21 => pt_const::TI.len(),
            22 => pt_const::V_.len(),
            23 => pt_const::CR.len(),
            24 => pt_const::MN.len(),
            25 => pt_const::FE.len(),
            26 => pt_const::CO.len(),
            27 => pt_const::NI.len(),
            28 => pt_const::CU.len(),
            29 => pt_const::ZN.len(),
            30 => pt_const::GA.len(),
            31 => pt_const::GE.len(),
            32 => pt_const::AS.len(),
            33 => pt_const::SE.len(),
            34 => pt_const::BR.len(),
            35 => pt_const::KR.len(),
            36 => pt_const::RB.len(),
            37 => pt_const::SR.len(),
            38 => pt_const::Y_.len(),
            39 => pt_const::ZR.len(),
            40 => pt_const::NB.len(),
            41 => pt_const::MO.len(),
            42 => pt_const::TC.len(),
            43 => pt_const::RU.len(),
            44 => pt_const::RH.len(),
            45 => pt_const::PD.len(),
            46 => pt_const::AG.len(),
            47 => pt_const::CD.len(),
            48 => pt_const::IN.len(),
            49 => pt_const::SN.len(),
            50 => pt_const::SB.len(),
            51 => pt_const::TE.len(),
            52 => pt_const::I_.len(),
            53 => pt_const::XE.len(),
            54 => pt_const::CS.len(),
            55 => pt_const::BA.len(),
            56 => pt_const::LA.len(),
            57 => pt_const::CE.len(),
            58 => pt_const::PR.len(),
            59 => pt_const::ND.len(),
            60 => pt_const::PM.len(),
            61 => pt_const::SM.len(),
            62 => pt_const::EU.len(),
            63 => pt_const::GD.len(),
            64 => pt_const::TB.len(),
            65 => pt_const::DY.len(),
            66 => pt_const::HO.len(),
            67 => pt_const::ER.len(),
            68 => pt_const::TM.len(),
            69 => pt_const::YB.len(),
            70 => pt_const::LU.len(),
            71 => pt_const::HF.len(),
            72 => pt_const::TA.len(),
            73 => pt_const::W_.len(),
            74 => pt_const::RE.len(),
            75 => pt_const::OS.len(),
            76 => pt_const::IR.len(),
            77 => pt_const::PT.len(),
            78 => pt_const::AU.len(),
            79 => pt_const::HG.len(),
            80 => pt_const::TL.len(),
            81 => pt_const::PB.len(),
            82 => pt_const::BI.len(),
            83 => pt_const::PO.len(),
            84 => pt_const::AT.len(),
            85 => pt_const::RN.len(),
            86 => pt_const::FR.len(),
            87 => pt_const::RA.len(),
            88 => pt_const::AC.len(),
            89 => pt_const::TH.len(),
            90 => pt_const::PA.len(),
            91 => pt_const::U_.len(),

            // custom cases for transuranic elements
            92 => pt_const::NP.len() + 1,
            93 => pt_const::PU.len() + 1,
            _ => {
                panic!("unknown element number: {}", elem_nr)
            }
        }
    }
    pub fn nr_to_next(elem_nr: usize) -> &'static [usize] {
        match elem_nr {
            0 => pt_const::H__N,
            1 => pt_const::HE_N,
            2 => pt_const::LI_N,
            3 => pt_const::BE_N,
            4 => pt_const::B__N,
            5 => pt_const::C__N,
            6 => pt_const::N__N,
            7 => pt_const::O__N,
            8 => pt_const::F__N,
            9 => pt_const::NE_N,
            10 => pt_const::NA_N,
            11 => pt_const::MG_N,
            12 => pt_const::AL_N,
            13 => pt_const::SI_N,
            14 => pt_const::P__N,
            15 => pt_const::S__N,
            16 => pt_const::CL_N,
            17 => pt_const::AR_N,
            18 => pt_const::K__N,
            19 => pt_const::CA_N,
            20 => pt_const::SC_N,
            21 => pt_const::TI_N,
            22 => pt_const::V__N,
            23 => pt_const::CR_N,
            24 => pt_const::MN_N,
            25 => pt_const::FE_N,
            26 => pt_const::CO_N,
            27 => pt_const::NI_N,
            28 => pt_const::CU_N,
            29 => pt_const::ZN_N,
            30 => pt_const::GA_N,
            31 => pt_const::GE_N,
            32 => pt_const::AS_N,
            33 => pt_const::SE_N,
            34 => pt_const::BR_N,
            35 => pt_const::KR_N,
            36 => pt_const::RB_N,
            37 => pt_const::SR_N,
            38 => pt_const::Y__N,
            39 => pt_const::ZR_N,
            40 => pt_const::NB_N,
            41 => pt_const::MO_N,
            42 => pt_const::TC_N,
            43 => pt_const::RU_N,
            44 => pt_const::RH_N,
            45 => pt_const::PD_N,
            46 => pt_const::AG_N,
            47 => pt_const::CD_N,
            48 => pt_const::IN_N,
            49 => pt_const::SN_N,
            50 => pt_const::SB_N,
            51 => pt_const::TE_N,
            52 => pt_const::I__N,
            53 => pt_const::XE_N,
            54 => pt_const::CS_N,
            55 => pt_const::BA_N,
            56 => pt_const::LA_N,
            57 => pt_const::CE_N,
            58 => pt_const::PR_N,
            59 => pt_const::ND_N,
            60 => pt_const::PM_N,
            61 => pt_const::SM_N,
            62 => pt_const::EU_N,
            63 => pt_const::GD_N,
            64 => pt_const::TB_N,
            65 => pt_const::DY_N,
            66 => pt_const::HO_N,
            67 => pt_const::ER_N,
            68 => pt_const::TM_N,
            69 => pt_const::YB_N,
            70 => pt_const::LU_N,
            71 => pt_const::HF_N,
            72 => pt_const::TA_N,
            73 => pt_const::W__N,
            74 => pt_const::RE_N,
            75 => pt_const::OS_N,
            76 => pt_const::IR_N,
            77 => pt_const::PT_N,
            78 => pt_const::AU_N,
            79 => pt_const::HG_N,
            80 => pt_const::TL_N,
            81 => pt_const::PB_N,
            82 => pt_const::BI_N,
            83 => pt_const::PO_N,
            84 => pt_const::AT_N,
            85 => pt_const::RN_N,
            86 => pt_const::FR_N,
            87 => pt_const::RA_N,
            88 => pt_const::AC_N,
            89 => pt_const::TH_N,
            90 => pt_const::PA_N,
            91 => pt_const::U__N,
            // custom cases for transuranic elements
            92 => pt_const::NP_N,
            93 => pt_const::PU_N,
            _ => panic!("unknown element number: {}", elem_nr),
        }
    }
}

fn look_and_say_once(input: String) -> String {
    let mut chars = input.chars();
    let mut digit = chars.next().unwrap();
    let mut count = 1;
    let mut res = "".to_string();
    for c in chars {
        if c == digit {
            count += 1;
        } else {
            res = format!("{}{}{}", res, count, digit);
            digit = c;
            count = 1;
        }
    }
    return format!("{}{}{}", res, count, digit);
}

mod rules {
    const B1: u8 = b'1';
    const B2: u8 = b'2';
    const B3: u8 = b'3';
    // single rules assume that b[i] == b2
    fn single_rule_1(b: &[u8], l: usize, i: usize) -> bool {
        i + 2 < l && b[i + 1] == B1 && b[i + 1] != b[i + 2] && (i + 3 >= l || b[i + 2] != b[i + 3])
    }
    fn single_rule_2(b: &[u8], l: usize, i: usize) -> bool {
        i + 3 < l && b[i + 1] == B1 && b[i + 2] == B1 && b[i + 3] == B1
    }
    fn single_rule_3(b: &[u8], l: usize, i: usize) -> bool {
        i + 1 < l
            && b[i + 1] == B3
            && (i + 2 >= l
                || (b[i + 2] != B3 && (i + 4 >= l || b[i + 3] != b[i + 2] || b[i + 4] != b[i + 2])))
    }
    pub fn single_rule(b: &[u8], l: usize, i: usize) -> bool {
        single_rule_1(b, l, i) || single_rule_2(b, l, i) || single_rule_3(b, l, i)
    }
    // double rule assumes that b[i] != b2
    pub fn double_rule(b: &[u8], l: usize, i: usize) -> bool {
        i + 3 < l && b[i + 1] == B2 && b[i + 2] == B2 && single_rule(b, l, i + 2)
    }
    pub fn end_rule(b: &[u8], l: usize) -> bool {
        b[l - 3] != B2 && b[l - 2] == B2 && b[l - 1] == B2
    }
}

fn parse_elems(only_elements: &str) -> Vec<BigUint> {
    // find all splits according to "The Splitting Theorem", see
    // https://web.archive.org/web/20061224154744/http://www.uam.es/personal_pdi/ciencias/omartin/Biochem.PDF
    let b = only_elements.as_bytes();
    let l = b.len();
    let mut splits: Vec<usize> = Vec::new();
    for i in 0..b.len() {
        if b[i] == b'2' && rules::single_rule(b, l, i) {
            splits.push(i + 1);
        } else if b[i] != b'2' && rules::double_rule(b, l, i) {
            splits.push(i + 1);
        }
    }
    if rules::end_rule(b, l) {
        splits.push(l - 2);
    }

    // records how many times an element was found among the splits
    let mut arr = Vec::new();
    arr.resize(94, BigUint::new(Vec::new()));

    // first split
    arr[pt::str_to_nr(only_elements.get(0..splits[0]).unwrap())] += 1 as u32;

    for i in 0..(splits.len() - 1) {
        let s = only_elements.get(splits[i]..splits[i + 1]).unwrap();
        arr[pt::str_to_nr(s)] += 1 as u32;
    }
    // last split
    arr[pt::str_to_nr(only_elements.get(splits[splits.len() - 1]..).unwrap())] += 1 as u32;

    return arr;
}

fn smart_look_and_say(elems: Vec<BigUint>) -> Vec<BigUint> {
    let mut new_elems = Vec::new();
    new_elems.resize(94, BigUint::new(Vec::new()));
    for i in 0..elems.len() {
        for next_elem in pt::nr_to_next(i) {
            new_elems[*next_elem] += &elems[i];
        }
    }
    return new_elems;
}

fn repeat_smart_las(elems: Vec<BigUint>, num_reps: i32) -> Vec<BigUint> {
    let mut new_elems = elems;
    for _ in 0..num_reps {
        new_elems = smart_look_and_say(new_elems);
    }
    return new_elems;
}

fn calc_len(elems: &Vec<BigUint>) -> BigUint {
    let mut count = BigUint::new(Vec::new());
    for i in 0..elems.len() {
        count += &elems[i] * pt::nr_to_len(i);
    }
    return count;
}

fn look_and_say(input: &str, n: i32) {
    let mut only_elements = input.to_string();
    for _ in 0..24 {
        only_elements = look_and_say_once(only_elements)
    }
    let elems = parse_elems(&only_elements);
    let elems = repeat_smart_las(elems, n - 24);
    let length = calc_len(&elems);
    println!("length of sequence after {} runs: {}", n, length);
}

fn main() {
    pt::generate_periodic_table();

    let input = include_str!("../input/input.txt");

    look_and_say(input, 40);

    look_and_say(input, 50);

    look_and_say(input, 1000);

    // takes about 15 seconds with optimized compilation
    look_and_say(input, 200_000);
}
