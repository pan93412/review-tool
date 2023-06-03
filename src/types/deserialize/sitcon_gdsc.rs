//! The SITCON@GDSC style of CSV.

use crate::types::{self, Author};

#[derive(serde::Deserialize)]
pub struct SitconGdscDifficulty(String);

#[derive(serde::Deserialize)]
pub struct SitconGdscFormat {
    #[serde(rename = "您的姓名 / 暱稱 Name")]
    pub name: String,
    #[serde(rename = "題目 Title")]
    pub title: String,
    #[serde(rename = "投稿類型 Type")]
    pub post_type: String,
    #[serde(rename = "摘要 Abstract")]
    pub abstract_: String,
    #[serde(rename = "自我介紹 Self Introduction")]
    pub self_introduction: String,
    #[serde(rename = "目標受眾 Target Audience")]
    pub target_audience: String,
    #[serde(rename = "內容難易度 Difficulty")]
    pub difficulty: SitconGdscDifficulty,
    #[serde(rename = "詳細說明 Description")]
    pub description: String,
    #[serde(rename = "其他投稿相關補充資料 Supplemental materials")]
    pub supplemental_materials: String,
}

impl From<SitconGdscFormat> for types::Manuscript {
    fn from(src: SitconGdscFormat) -> Self {
        Self {
            title: src.title,
            type_: src.post_type,
            abstract_: src.abstract_,
            audience: src.target_audience,
            difficulty: src.difficulty.into(),
            description: src.description,
            extra: src.supplemental_materials,
            author: Author {
                name: src.name,
                description: src.self_introduction,
            },
        }
    }
}

impl From<SitconGdscDifficulty> for types::Difficulty {
    fn from(src: SitconGdscDifficulty) -> Self {
        match src.0.to_ascii_lowercase().as_str() {
            "容易 easy" => Self::Easy,
            "中等 medium" => Self::Medium,
            "困難 hard" => Self::Hard,
            "beginner" => Self::Easy,
            "intermediate" => Self::Medium,
            "advanced" => Self::Hard,
            "入門" => Self::Easy,
            "初級" => Self::Easy,
            "中級" => Self::Medium,
            "中階" => Self::Medium,
            "中等" => Self::Medium,
            "進階" => Self::Hard,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::types::{Author, Difficulty, Manuscript};

    #[test]
    fn test_difficulty_into() {
        let raw = [
            "中階", "中階", "入門", "入門", "入門", "中階", "入門", "入門", "入門", "Beginner",
            "入門", "入門", "入門", "入門", "中階", "入門", "入門", "入門", "中階",
        ];

        let expected = [
            Difficulty::Medium,
            Difficulty::Medium,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Easy,
            Difficulty::Medium,
        ];

        for (raw, expected) in raw.iter().zip(expected.iter()) {
            let actual = super::SitconGdscDifficulty(raw.to_string()).into();
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn test_deserialize() {
        let input = indoc::indoc! { r#"
        您的姓名 / 暱稱 Name,題目 Title,投稿類型 Type,摘要 Abstract,自我介紹 Self Introduction,目標受眾 Target Audience,內容難易度 Difficulty,詳細說明 Description,其他投稿相關補充資料 Supplemental materials
        Kyle Lin,從V和Zig中探索編譯期間運算的實際應用,general (30mins),本議程將會介紹編譯期間運算 (compile time evaluation)的概念以及如何應用在實際的產品上。,編譯器研究者，V語言編譯器開發社群成員，TWKUG志工。主要致力於開源文化的推廣及實踐。,對於meta programming有興趣之開發者,中階,近年來各式各樣的新興語言為了和其他語言的特性分庭抗禮，紛紛加入了各式各樣meta programming來吸引世界各地的程式語言愛好者前來使用，而在此之中編譯期間運算(compile time evalution)也隨之被多個程式語言實作出來，並各自有著不同的特點。在本議程，將會帶領讀者理解編譯期間運算的概念、使用情境、以及如何應用在實際的產品上。,
        FKT,Django REST View 到底有那些寫法哩 我好像都只用 APIView 呢?,general (30mins),Django 是一個開源的 Python 後端框架，在進行前後端分離時，會額外安裝 Django REST Framework 在我們的原始 Django 應用程式上，但各位知道嗎 Django REST Framework 在視圖上提供了多種封裝完善的類別供開發者進行使用。,HI 我是FKT 是現任2022~2023 NYUST GDSC Lead 擅長Backend and DevOps但其他領域也有興趣所以會出一些其他的專案或文章 個人部落格 https://www.sql-fan9704.eu.org/ GitHub https://github.com/fan9704,Django REST Framework 入門開發者,中階,Django 是一個開源的 Python 後端框架，在進行前後端分離時，會額外安裝 Django REST Framework 在我們的原始 Django 應用程式上，但各位知道嗎 Django REST Framework 在視圖上提供了多種封裝完善的類別供開發者進行使用，在 Django REST Framework 提供的視圖工具中我們搭配使用在 Django 中方便應用程式對 JSON 資料進行的打包與封裝的 Serializer 進行序列化與反序列化，短短 10 行視圖的 Python 程式碼也許能夠幫助你完成 C R U D 的資料操作，在這個議程中你會認識各種 CBV(Class Based View) 有 APIView，ViewSets，Mixin 及 GenericAPIView 的基本關係與實作方法。,
        uccu,從只會 Docker 到上手 Kubernetes,general (30mins),學生碰不到，但企業都在用的 Kubernetes 究竟該如何學習。本場議程中，講者將分享自己從僅會使用 Docker 建立容器，到後來了解並掌握 Kubernetes 的經驗，並提供學習方式，引導大家進入 Kubernetes 的世界。,熱愛 DevOps 技術與 Hackathon 文化的碩士生,學生、有使用 Container 經驗者、想學習 Kubernetes 的人,中階,Outline1. 什麼是 Kubernetes (8mins)2. 為何要學習 Kubernetes (12mins)3. 如何學習 Kubernetes (5mins)4. Q&A (5mins),iThome鐵人賽講者為 2022 鐵人賽 DevOps 佳作https://ithelp.ithome.com.tw/users/20139235梅竹黑客松2022講者為 2022 梅竹黑客松 梅竹大獎第一名https://2022.meichuhackathon.org
        火山 / Kazan,假如我年少有為不自卑——作為學生，你也可以進行開源貢獻！,general (30mins),作為學生，很多時候我們並不像大多社群成員一樣可能擁有高深的技術或豐富的知識，但並不代表我們不能對開源社群做出貢獻！「**每個人都有自己能做的事情**」，這就是社群最棒的地方，讓我透過幾個實際參與的例子，告訴各位作為學生我們能怎麼進行貢獻！,從社會組半路出家，熱愛開源，目前是個學店大二學生，喜歡資訊安全、軟體開發、Linux，自許能成為一個開源推廣與貢獻的工作者。個人網站：https://kazan.tw,對 Open Source 有興趣、想參與貢獻但是怕自己技術不夠強的學生族群,入門,議程會簡述各種參與貢獻的方式（包含但不限於參與社群、翻譯、發 PR、寫簡單的 side projiect 等），並鼓勵大家從學生時期就多多進行開源貢獻，累積經驗的同時也能更加深刻的理解開源精神。,簡報連結：https://docs.google.com/presentation/d/1UBqAPb6svlfCEQP3evZ_88HDGnySacHkrPcjoCWqoZI/edit?usp=sharing
        "# };

        let expected = [
            Manuscript {
                title: "從V和Zig中探索編譯期間運算的實際應用".into(),
                type_: "general (30mins)".into(),
                abstract_: "本議程將會介紹編譯期間運算 (compile time evaluation)的概念以及如何應用在實際的產品上。".into(),
                audience: "對於meta programming有興趣之開發者".into(),
                difficulty: Difficulty::Medium,
                description: "近年來各式各樣的新興語言為了和其他語言的特性分庭抗禮，紛紛加入了各式各樣meta programming來吸引世界各地的程式語言愛好者前來使用，而在此之中編譯期間運算(compile time evalution)也隨之被多個程式語言實作出來，並各自有著不同的特點。在本議程，將會帶領讀者理解編譯期間運算的概念、使用情境、以及如何應用在實際的產品上。".into(),
                extra: String::new(),
                author: Author {
                    name: "Kyle Lin".into(),
                    description: "編譯器研究者，V語言編譯器開發社群成員，TWKUG志工。主要致力於開源文化的推廣及實踐。".into(),
                },
            },
            Manuscript {
                title: "Django REST View 到底有那些寫法哩 我好像都只用 APIView 呢?".into(),
                type_: "general (30mins)".into(),
                abstract_: "Django 是一個開源的 Python 後端框架，在進行前後端分離時，會額外安裝 Django REST Framework 在我們的原始 Django 應用程式上，但各位知道嗎 Django REST Framework 在視圖上提供了多種封裝完善的類別供開發者進行使用。".into(),
                audience: "Django REST Framework 入門開發者".into(),
                difficulty: crate::types::Difficulty::Medium,
                description: "Django 是一個開源的 Python 後端框架，在進行前後端分離時，會額外安裝 Django REST Framework 在我們的原始 Django 應用程式上，但各位知道嗎 Django REST Framework 在視圖上提供了多種封裝完善的類別供開發者進行使用，在 Django REST Framework 提供的視圖工具中我們搭配使用在 Django 中方便應用程式對 JSON 資料進行的打包與封裝的 Serializer 進行序列化與反序列化，短短 10 行視圖的 Python 程式碼也許能夠幫助你完成 C R U D 的資料操作，在這個議程中你會認識各種 CBV(Class Based View) 有 APIView，ViewSets，Mixin 及 GenericAPIView 的基本關係與實作方法。".into(),
                extra: "".into(),
                author: Author {
                    name: "FKT".into(),
                    description: "HI 我是FKT 是現任2022~2023 NYUST GDSC Lead 擅長Backend and DevOps但其他領域也有興趣所以會出一些其他的專案或文章 個人部落格 https://www.sql-fan9704.eu.org/ GitHub https://github.com/fan9704".into(),
                }
            },
            Manuscript {
                title: "從只會 Docker 到上手 Kubernetes".into(),
                type_: "general (30mins)".into(),
                abstract_: "學生碰不到，但企業都在用的 Kubernetes 究竟該如何學習。本場議程中，講者將分享自己從僅會使用 Docker 建立容器，到後來了解並掌握 Kubernetes 的經驗，並提供學習方式，引導大家進入 Kubernetes 的世界。".into(),
                audience: "學生、有使用 Container 經驗者、想學習 Kubernetes 的人".into(),
                difficulty: crate::types::Difficulty::Medium,
                description: "Outline1. 什麼是 Kubernetes (8mins)2. 為何要學習 Kubernetes (12mins)3. 如何學習 Kubernetes (5mins)4. Q&A (5mins)".into(),
                extra: "iThome鐵人賽講者為 2022 鐵人賽 DevOps 佳作https://ithelp.ithome.com.tw/users/20139235梅竹黑客松2022講者為 2022 梅竹黑客松 梅竹大獎第一名https://2022.meichuhackathon.org".into(),
                author: Author {
                    name: "uccu".into(),
                    description: "熱愛 DevOps 技術與 Hackathon 文化的碩士生".into(),
                }
            },
            Manuscript {
                title: "假如我年少有為不自卑——作為學生，你也可以進行開源貢獻！".into(),
                type_: "general (30mins)".into(),
                abstract_: "作為學生，很多時候我們並不像大多社群成員一樣可能擁有高深的技術或豐富的知識，但並不代表我們不能對開源社群做出貢獻！「**每個人都有自己能做的事情**」，這就是社群最棒的地方，讓我透過幾個實際參與的例子，告訴各位作為學生我們能怎麼進行貢獻！".into(),
                audience: "對 Open Source 有興趣、想參與貢獻但是怕自己技術不夠強的學生族群".into(),
                difficulty: crate::types::Difficulty::Easy,
                description: "議程會簡述各種參與貢獻的方式（包含但不限於參與社群、翻譯、發 PR、寫簡單的 side projiect 等），並鼓勵大家從學生時期就多多進行開源貢獻，累積經驗的同時也能更加深刻的理解開源精神。".into(),
                extra: "簡報連結：https://docs.google.com/presentation/d/1UBqAPb6svlfCEQP3evZ_88HDGnySacHkrPcjoCWqoZI/edit?usp=sharing".into(),
                author: Author {
                    name: "火山 / Kazan".into(),
                    description: "從社會組半路出家，熱愛開源，目前是個學店大二學生，喜歡資訊安全、軟體開發、Linux，自許能成為一個開源推廣與貢獻的工作者。個人網站：https://kazan.tw".into(),
                }
            }
        ];

        let reader = Cursor::new(input.as_bytes());
        let deserialized_result =
            super::super::deserialize_internal::<super::SitconGdscFormat>(reader);
        let deserialized_result = deserialized_result.unwrap();

        assert_eq!(deserialized_result, expected);
    }
}
