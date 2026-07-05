use serde::{Deserialize, Serialize};

use crate::{account::Account, connection::Connection, error::Error};

/// Represents a complete SimpleFIN API response containing accounts, connections, and errors.
///
/// This is the primary data structure returned by SimpleFIN v2 endpoints.
/// See the [crate-level documentation](crate) for usage examples.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(bound(
    serialize = "AccountExtraT: Serialize, TransactionExtraT: Serialize",
    deserialize = "AccountExtraT: Deserialize<'de>, TransactionExtraT: Deserialize<'de>"
))]
pub struct AccountSet<AccountExtraT = (), TransactionExtraT = ()> {
    pub errlist: Vec<Error>,
    #[deprecated = "Use errlist"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<String>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub accounts: Vec<Account<AccountExtraT, TransactionExtraT>>,
}

impl<AccountExtraT, TransactionExtraT> Default for AccountSet<AccountExtraT, TransactionExtraT> {
    fn default() -> Self {
        Self {
            errlist: Vec::new(),
            #[allow(deprecated)]
            errors: None,
            connections: Vec::new(),
            accounts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        account::{Account, AccountId, AccountName, Currency},
        connection::{ConnectionId, ConnectionName, OrganizationId, OrganizationUrl, SimplefinUrl},
        deserialize_date, serialize_date,
        transaction::{Transaction, TransactionId},
    };

    use super::*;
    use chrono::{DateTime, Utc};
    use rstest::rstest;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct AccountExtra {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date",
            rename = "account-open-date"
        )]
        pub account_open_date: DateTime<Utc>,
    }

    #[rstest]
    #[case(
r#"{
  "errlist": [],
  "connections": [
    {
      "conn_id": "CON-1122121298398234234",
      "name": "My Bank - Jill",
      "org_id": "INST-1298391823-129381928391823",
      "org_url": "https://mybank.com/",
      "sfin_url": "https://sfin.mybank.com/"
    }
  ],
  "accounts": [
    {
      "id": "2930002",
      "name": "Savings",
      "conn_id": "CON-1122121298398234234",
      "currency": "USD",
      "balance": "100.23",
      "available-balance": "75.23",
      "balance-date": 978366153,
      "transactions": [
        {
          "id": "12394832938403",
          "posted": 793090572,
          "amount": "-33293.43",
          "description": "Uncle Frank's Bait Shop"
        }
      ],
      "extra": {
        "account-open-date": 978360153
      }
    }
  ]
}"#,
        AccountSet::<_, _> {
            errlist: vec![],
            connections: vec![
                Connection {
                    connection_id: ConnectionId::new("CON-1122121298398234234"),
                    name: ConnectionName::new("My Bank - Jill"),
                    organization_id: OrganizationId::new("INST-1298391823-129381928391823"),
                    organization_url: Some(OrganizationUrl::new("https://mybank.com").unwrap()),
                    simplefin_url: SimplefinUrl::new("https://sfin.mybank.com").unwrap(),
                },
            ],
            accounts: vec![
                Account {
                    account_id: AccountId::new("2930002"),
                    name: AccountName::new("Savings"),
                    connection_id: ConnectionId::new("CON-1122121298398234234"),
                    currency: Currency::new("USD"),
                    balance: 100.23,
                    available_balance: Some(75.23),
                    balance_date: DateTime::from_timestamp_secs(978366153).unwrap(),
                    transactions: vec![
                        Transaction {
                            transaction_id:TransactionId::new("12394832938403"),
                            posted: DateTime::from_timestamp_secs(793090572).unwrap(),
                            amount: -33293.43,
                            description: "Uncle Frank's Bait Shop".to_string(),
                            transacted_at: None,
                            pending: None,
                            extra: None,
                        }
                    ],
                    extra: Some(AccountExtra {
                        account_open_date: DateTime::from_timestamp_secs(978360153).unwrap(),
                    })
                }
            ],
            ..Default::default()
        }
    )]
    fn test_examples(#[case] input: &str, #[case] expected: AccountSet<AccountExtra>) {
        let deserialized: AccountSet<_, _> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
