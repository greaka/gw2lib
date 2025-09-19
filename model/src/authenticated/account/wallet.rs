use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::{Authenticated, Endpoint, FixedEndpoint, misc::currencies::CurrencyId};

type InnerWallet = HashMap<CurrencyId, u32>;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Wallet(#[serde(with = "internal_wallet")] pub InnerWallet);

impl Endpoint for Wallet {
    type Authenticated = Authenticated;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wallet";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Wallet {}

impl Deref for Wallet {
    type Target = InnerWallet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wallet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod internal_wallet {
    use std::collections::HashMap;

    use serde::{
        Deserialize, Deserializer, Serialize,
        de::{SeqAccess, Visitor},
        ser::Serializer,
    };

    use super::InnerWallet;
    use crate::misc::currencies::CurrencyId;

    #[derive(Debug, Serialize, Deserialize)]
    struct InternalWallet {
        id: CurrencyId,
        value: u32,
    }

    pub fn serialize<S>(map: &InnerWallet, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(
            map.iter()
                .map(|(&k, &v)| InternalWallet { id: k, value: v }),
        )
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<InnerWallet, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InternalWalletVisitor;

        impl<'de> Visitor<'de> for InternalWalletVisitor {
            type Value = InnerWallet;

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
