//! The rank items of SITCON x GDSC in Pan rules.

/// 主題相關：和學生、社群以及程式相關的議題，且與 FLOSS（自由/開放原始碼軟體）相關。
pub mod subject {
    use serde::{Serialize, Deserialize};

    use crate::types::rank::{ItemGroup, StandardChoice, Item};

    super::new_rank!(StudentRelated, "和學生相關", None);
    super::new_rank!(CommunityRelated, "和社群相關", None);
    super::new_rank!(CodingRelated, "和程式相關", None);
    super::new_rank!(FLOSSRelated, "和開源相關", None);

    #[derive(Default, Serialize, Deserialize)]
    pub struct Group {
        pub student_related: StudentRelated,
        pub community_related: CommunityRelated,
        pub coding_related: CodingRelated,

        pub floss_related: FLOSSRelated,
    }

    impl ItemGroup for Group {
        fn score(&self) -> f64 {
            let topic_score = [
                self.student_related.choice(),
                self.community_related.choice(),
                self.coding_related.choice(),
            ].iter().map(|c| {
                {
                    match c {
                        StandardChoice::Full => 3.5,
                        StandardChoice::Partial => 3.0,
                        StandardChoice::Maybe => 1.5,
                        StandardChoice::No => 0.0,
                    }
                }
            }).sum::<f64>().min(7.0);

            let floss_score = {
                match self.floss_related.choice() {
                    StandardChoice::Full => 3.0,
                    StandardChoice::Partial => 1.5,
                    StandardChoice::Maybe => 0.0,
                    StandardChoice::No => 0.0,
                }
            };

            topic_score + floss_score
        }

        fn score_description(&self) -> Option<String> {
            let result = format!(
                "和學生{stu}、社群{com}以及程式{cod}相關的議題，且與 FLOSS（自由/開放原始碼軟體）相關{fls}。",
                stu=self.student_related.choice().as_emoji(),
                com=self.community_related.choice().as_emoji(),
                cod=self.coding_related.choice().as_emoji(),
                fls=self.floss_related.choice().as_emoji(),
            );

            Some(result)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Group {
    pub subject: subject::Group,
}

macro_rules! new_rank {
    ($name:ident, $display_name:expr, $description:expr) => {
        #[derive(Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $name {
            comment: Option<String>,
            choice: crate::types::rank::StandardChoice,
        }

        impl crate::types::rank::Item for $name {
            fn name(&self) -> &str {
                $display_name
            }

            fn description(&self) -> Option<&str> {
                $description
            }

            fn choice(&self) -> crate::types::rank::StandardChoice {
                self.choice
            }

            fn choice_mut(&mut self) -> &mut crate::types::rank::StandardChoice {
                &mut self.choice
            }

            fn comment(&self) -> Option<&str> {
                self.comment.as_deref()
            }

            fn comment_mut(&mut self) -> &mut Option<String> {
                &mut self.comment
            }
        }
    };
}
pub(self) use new_rank;
use serde::{Serialize, Deserialize};
