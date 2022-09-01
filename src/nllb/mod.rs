use crate::{
    m2m_100::M2M100Generator,
    pipelines::translation::Language::{self, *},
};

pub struct NLLBResources;
pub struct NLLBConfigResources;
pub struct NLLBVocabResources;
pub struct NLLBMergeResources;
pub struct NLLBLanguages;
pub struct NLLBSpecialMap;

impl NLLBConfigResources {
    pub const NLLB_600M_DISTILLED: (&'static str, &'static str) = (
        "nllb200-distilled-600m/config",
        "https://huggingface.co/facebook/nllb-200-distilled-600M/raw/main/config.json",
    );

    pub const NLLB_1_3B: (&'static str, &'static str) = (
        "nllb200-1_3b/config",
        "https://huggingface.co/facebook/nllb-200-1.3B/raw/main/config.json",
    );

    pub const NLLB_3_3B: (&'static str, &'static str) = (
        "nllb200-3_3b/config",
        "https://huggingface.co/facebook/nllb-200-3.3B/raw/main/config.json",
    );
}

impl NLLBVocabResources {
    pub const NLLB_600M_DISTILLED: (&'static str, &'static str) = (
        "nllb200-distilled-600m/vocab",
        "https://huggingface.co/facebook/nllb-200-distilled-600M/resolve/main/tokenizer.json",
    );

    pub const NLLB_1_3B: (&'static str, &'static str) = (
        "nllb200-1_3b/vocab",
        "https://huggingface.co/facebook/nllb-200-1.3B/resolve/main/tokenizer.json",
    );

    pub const NLLB_3_3B: (&'static str, &'static str) = (
        "nllb200-3_3b/vocab",
        "https://huggingface.co/facebook/nllb-200-3.3B/resolve/main/tokenizer.json",
    );
}

impl NLLBMergeResources {
    pub const NLLB_600M_DISTILLED: (&'static str, &'static str) = (
        "nllb200-distilled-600m/merge",
        "https://huggingface.co/facebook/nllb-200-distilled-600M/resolve/main/sentencepiece.bpe.model",
    );

    pub const NLLB_1_3B: (&'static str, &'static str) = (
        "nllb200-1_3b/merge",
        "https://huggingface.co/facebook/nllb-200-1.3B/resolve/main/sentencepiece.bpe.model",
    );

    pub const NLLB_3_3B: (&'static str, &'static str) = (
        "nllb200-3_3b/merge",
        "https://huggingface.co/facebook/nllb-200-3.3B/resolve/main/sentencepiece.bpe.model",
    );
}

impl NLLBSpecialMap {
    pub const NLLB_600M_DISTILLED: (&'static str, &'static str) = (
        "nllb200-distilled-600m/special",
        "htps://huggingface.co/facebook/nllb-200-distilled-600M/raw/main/special_tokens_map.json",
    );

    pub const NLLB_1_3B: (&'static str, &'static str) = (
        "nllb200-1_3b/special",
        "https://huggingface.co/facebook/nllb-200-1.3B/raw/main/special_tokens_map.json",
    );

    pub const NLLB_3_3B: (&'static str, &'static str) = (
        "nllb200-3_3b/special",
        "https://huggingface.co/facebook/nllb-200-3.3B/raw/main/special_tokens_map.json",
    );
}

impl NLLBLanguages {
    #[rustfmt::skip]
    pub const NLLB_600M_DISTILLED: [Language; 195] = [
        Achinese, MesopotamianArabic, TaizziAdeniArabic, TunisianArabic,
        Afrikaans, SouthLevantineArabic, Akan, ToskAlbanian, Amharic,
        NorthLevantineArabic, StandardArabic, NajdiArabic, MoroccanArabic, EgyptianArabic,
        Assamese, Asturian, Awadhi, CentralAymara, SouthAzerbaijani,
        NorthAzerbaijani, Bashkir, Bambara, Balinese, Belarusian, Bemba,
        Bengali, Bhojpuri, Banjar, Tibetan, Bosnian, Buginese, Bulgarian,
        Catalan, Cebuano, Czech, Chokwe, CentralKurdish, CrimeanTatar,
        Welsh, Danish, German, SouthwesternDinka, Dyula, Dzongkha, Greek,
        English, Esperanto, Estonian, Basque, Ewe, Faroese, Fijian, Finnish,
        Fon, French, Friulian, NigerianFulfulde, WestCentralOromo, ScottishGaelic,
        Irish, Galician, Guarani, Gujarati, Haitian, Hausa, Hebrew, Hindi,
        Chhattisgarhi, Croatian, Hungarian, Armenian, Igbo, Iloko, Indonesian,
        Icelandic, Italian, Javanese, Japanese, Kabyle, Kachin, Kamba, Kannada,
        Kashmiri, Georgian, Kazakh, Kabuverdianu, HalhMongolian, Khmer,
        Kikuyu, Kinyarwanda, Kirghiz, Kimbundu, NorthernKurdish, CentralKanuri,
        Kongo, Korean, Lao, Ligurian, Limburgan, Lingala, Lithuanian, Lombard,
        Latgalian, Luxembourgish, LubaLulua, Ganda, Luo, Lushai,
        StandardLatvian, Magahi, Maithili, Malayalam, Marathi, Minangkabau,
        Macedonian, Maltese, Manipuri, Mossi, Maori, Burmese, Dutch,
        NorwegianNynorsk, NorwegianBokmal, Nepali, Pedi, Nuer, Nyanja,
        Occitan, Odia, Pangasinan, Panjabi, Papiamento, SouthernPashto, IranianPersian,
        PlateauMalagasy, Polish, Portuguese, Dari, AyacuchoQuechua, Romanian,
        Rundi, Russian, Sango, Sanskrit, Santali, Sicilian, Shan,
        Sinhala, Slovak, Slovenian, Samoan, Shona, Sindhi, Somali,
        SouthernSotho, Spanish, Sardinian, Serbian, Swati, Sundanese, Swedish, Swahili,
        Silesian, Tamil, Tamasheq, Tatar, Telugu, Tajik, Tagalog, Thai,
        Tigrinya, TokPisin, Tswana, Tsonga, Turkmen, Tumbuka, Turkish,
        Twi, CentralAtlasTamazight, Uighur, Ukrainian, Umbundu, Urdu, NorthernUzbek, Venetian,
        Vietnamese, Waray, Wolof, Xhosa, EasternYiddish, Yoruba, YueChinese,
        Chinese, StandardMalay, Zulu,
    ];
}

pub type NLLBGenerator = M2M100Generator;
