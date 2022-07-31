use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{misc::currencies::CurrencyId, Endpoint, FixedEndpoint};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet(#[serde(with = "internal_wallet")] pub HashMap<CurrencyId, u32>);

impl Endpoint for Wallet {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wallet";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Wallet {}

mod internal_wallet {
    use std::collections::HashMap;

    use serde::{
        de::{SeqAccess, Visitor},
        ser::Serializer,
        Deserialize, Deserializer, Serialize,
    };

    use crate::misc::currencies::CurrencyId;

    #[derive(Debug, Serialize, Deserialize)]
    struct InternalWallet {
        id: CurrencyId,
        value: u32,
    }

    pub fn serialize<S>(map: &HashMap<CurrencyId, u32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(
            map.iter()
                .map(|(&k, &v)| InternalWallet { id: k, value: v }),
        )
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<CurrencyId, u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InternalWalletVisitor;

        impl<'de> Visitor<'de> for InternalWalletVisitor {
            type Value = HashMap<CurrencyId, u32>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of InternalWallet")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut map = HashMap::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(item) = seq.next_element::<InternalWallet>()? {
                    map.insert(item.id, item.value);
                }

                Ok(map)
            }
        }

        deserializer.deserialize_seq(InternalWalletVisitor)
    }
}
