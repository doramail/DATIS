pub mod aws;
pub mod gcloud;
pub mod win;

use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub enum TextToSpeechProvider {
    GoogleCloud { voice: gcloud::VoiceKind },
    AmazonWebServices { voice: aws::VoiceKind },
    Windows { voice: Option<win::VoiceKind> },
}

#[derive(Clone)]
pub enum TextToSpeechConfig {
    GoogleCloud(gcloud::GoogleCloudConfig),
    AmazonWebServices(aws::AmazonWebServicesConfig),
    Windows(win::WindowsConfig),
}

impl Default for TextToSpeechProvider {
    fn default() -> Self {
        TextToSpeechProvider::Windows { voice: None }
    }
}

impl fmt::Debug for TextToSpeechProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            TextToSpeechProvider::GoogleCloud { voice } => {
                write!(f, "Google Cloud (Voice: {:?})", voice)
            }
            TextToSpeechProvider::AmazonWebServices { voice } => {
                write!(f, "Amazon Web Services (Voice: {:?})", voice)
            }
            TextToSpeechProvider::Windows { voice } => write!(
                f,
                "Windows built-in TTS (Voice: {:?})",
                voice.as_ref().map(|v| &**v).unwrap_or_else(|| "Default")
            ),
        }
    }
}

impl FromStr for TextToSpeechProvider {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        match *v.as_slice() {
            [prefix, voice] => match prefix {
                "GC" | "gc" => {
                    return Ok(TextToSpeechProvider::GoogleCloud {
                        voice: gcloud::VoiceKind::from_str(voice)?,
                    })
                }
                "AWS" | "aws" => {
                    return Ok(TextToSpeechProvider::AmazonWebServices {
                        voice: aws::VoiceKind::from_str(voice)?,
                    })
                }
                "WIN" | "win" => {
                    return Ok(TextToSpeechProvider::Windows {
                        voice: Some(win::VoiceKind::from_str(voice)?),
                    })
                }
                _ => {}
            },
            [voice] if !voice.is_empty() => {
                if voice == "WIN" || voice == "win" {
                    return Ok(TextToSpeechProvider::Windows { voice: None });
                } else {
                    return Ok(TextToSpeechProvider::GoogleCloud {
                        voice: gcloud::VoiceKind::from_str(voice)?,
                    });
                }
            }
            _ => {}
        }

        // fallback
        Ok(TextToSpeechProvider::default())
    }
}

#[cfg(test)]
mod test {
    mod tts_provider_from_str {
        use std::str::FromStr;

        use crate::tts::{aws, gcloud, TextToSpeechProvider};

        #[test]
        fn fallback_on_empty_string() {
            assert_eq!(
                TextToSpeechProvider::from_str("").unwrap(),
                TextToSpeechProvider::default()
            )
        }

        #[test]
        fn fallback_on_unknown_prefix() {
            assert_eq!(
                TextToSpeechProvider::from_str("UNK:foobar").unwrap(),
                TextToSpeechProvider::default()
            )
        }

        #[test]
        fn no_prefix_defaults_to_gcloud() {
            assert_eq!(
                TextToSpeechProvider::from_str("en-US-Wavenet-A").unwrap(),
                TextToSpeechProvider::GoogleCloud {
                    voice: gcloud::VoiceKind::EnUsWavenetA
                }
            )
        }

        #[test]
        fn prefix_gc() {
            assert_eq!(
                TextToSpeechProvider::from_str("GC:en-US-Wavenet-B").unwrap(),
                TextToSpeechProvider::GoogleCloud {
                    voice: gcloud::VoiceKind::EnUsWavenetB
                }
            )
        }

        #[test]
        fn gc_en_gp() {
            assert_eq!(
                TextToSpeechProvider::from_str("GC:en-GB-Standard-A").unwrap(),
                TextToSpeechProvider::GoogleCloud {
                    voice: gcloud::VoiceKind::EnGbStandardA
                }
            )
        }

        #[test]
        fn prefix_aws() {
            assert_eq!(
                TextToSpeechProvider::from_str("AWS:Brian").unwrap(),
                TextToSpeechProvider::AmazonWebServices {
                    voice: aws::VoiceKind::Brian
                }
            )
        }
    }
}
