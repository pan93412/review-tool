//! The rank items of SITCON x GDSC in Pan rules.

use serde::{Deserialize, Serialize};

/// 主題相關：和學生、社群以及程式相關的議題，且與 FLOSS（自由/開放原始碼軟體）相關。
pub mod subject {
    use crate::types::rank::{Item, ItemGroup, StandardChoice};

    super::new_rank!(StudentRelated, "和學生相關", None);
    super::new_rank!(CommunityRelated, "和社群相關", None);
    super::new_rank!(CodingRelated, "和程式相關", None);
    super::new_rank!(FlossRelated, "和開源相關", None);

    super::new_group!(
        StudentRelated,
        CommunityRelated,
        CodingRelated,
        FlossRelated
    );

    impl ItemGroup for Group {
        fn name(&self) -> &str {
            "主題相關性"
        }

        fn description(&self) -> Option<&str> {
            Some("和學生、社群以及程式相關的議題，且與 FLOSS（自由/開放原始碼軟體）相關。")
        }

        fn score(&self) -> f64 {
            let topic_score = [
                self.student_related.choice(),
                self.community_related.choice(),
                self.coding_related.choice(),
            ]
            .iter()
            .map(|c| match c {
                StandardChoice::Full => 3.5,
                StandardChoice::Partial => 3.0,
                StandardChoice::Maybe => 1.5,
                StandardChoice::No => 0.0,
            })
            .sum::<f64>()
            .min(7.0);

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

/// 表達能力：提供的資料是否有條理、文句暢通，以及提供資料之完整度。完整的資料能讓審稿委員更清楚了解演講細節。
pub mod expressive {
    use crate::types::rank::{Item, ItemGroup, StandardChoice};

    super::new_rank!(Organized, "資料有條理", None);
    super::new_rank!(Fluent, "文句暢通", None);
    super::new_rank!(Completeness, "資料完整度", None);

    super::new_group!(Organized, Fluent, Completeness);

    impl ItemGroup for Group {
        fn name(&self) -> &str {
            "表達能力"
        }

        fn description(&self) -> Option<&str> {
            Some("提供的資料是否有條理、文句暢通，以及提供資料之完整度。完整的資料能讓審稿委員更清楚了解演講細節。")
        }

        fn score(&self) -> f64 {
            // 三點各佔 3.3%，四捨五入

            [
                self.organized.choice(),
                self.fluent.choice(),
                self.completeness.choice(),
            ]
            .into_iter()
            .map(|c| match c {
                StandardChoice::Full => 3.3,
                StandardChoice::Partial => 2.5,
                StandardChoice::Maybe => 1.5,
                StandardChoice::No => 0.0,
            })
            .sum::<f64>()
            .round()
        }

        fn score_description(&self) -> Option<String> {
            Some(format!(
                "提供的資料是否有條理 {org}、文句暢通 {flu}，以及提供資料之完整度 {com}。",
                org = self.organized.choice.as_emoji(),
                flu = self.fluent.choice.as_emoji(),
                com = self.completeness.choice.as_emoji()
            ))
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Group {
    pub subject: subject::Group,
    pub expressive: expressive::Group,
}

macro_rules! new_rank {
    ($name:ident, $display_name:expr, $description:expr) => {
        ::paste::paste! {
            #[derive(Default, ::serde::Serialize, ::serde::Deserialize, Hash, Eq, PartialEq, Debug)]
            pub struct $name {
                comment: Option<String>,
                choice: crate::types::rank::StandardChoice,
            }
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

macro_rules! new_group {
    ($($entries:ident),+) => {
        ::paste::paste! {
            #[derive(Default, ::serde::Serialize, ::serde::Deserialize, Hash, Eq, PartialEq, Debug)]
            pub struct Group {
                $(
                    pub [< $entries:snake >]: $entries
                ),+
            }
        }
    }
}

pub(self) use {new_group, new_rank};
