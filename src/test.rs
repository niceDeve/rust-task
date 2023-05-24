use crate::{calculate_balance_changes, Balance, DenomDefinition, MultiSend, Coin};

// The Test_Case struct represents a single test case. It contains the original balances, definitions, and multi-send transaction data, as well as the expected result.
pub struct Test_Case {
    original_balances: Vec<Balance>,
    definitions: Vec<DenomDefinition>,
    multi_send_tx: MultiSend,
    result: Result<Vec<Balance>, String>,
}

// The compare_balances function compares two vectors of Balance structs, ignoring the order of the elements. This is because the order of the balances does not matter in this context.
fn compare_balances(_expected_balances: &Vec<Balance>, _result_balances: &Vec<Balance>) -> bool {
    if _expected_balances.len() != _result_balances.len() {
        return false;
    }

    let mut sorted_expected_balances = _expected_balances.clone();
    sorted_expected_balances.sort_by(|a, b| a.address.cmp(&b.address));
    let mut sorted_result_balances = _result_balances.clone();
    sorted_result_balances.sort_by(|a, b| a.address.cmp(&b.address));

    for i in 0..sorted_expected_balances.len() {
        if sorted_expected_balances[i].address != sorted_result_balances[i].address {
            return false;
        }
        let mut sorted_expected_coins = sorted_expected_balances[i].coins.clone();
        sorted_expected_coins.sort_by(|a, b| a.denom.cmp(&b.denom));
        let mut sorted_result_coins = sorted_result_balances[i].coins.clone();
        sorted_result_coins.sort_by(|a, b| a.denom.cmp(&b.denom));
        if sorted_expected_coins.len() != sorted_result_coins.len() {
            return false;
        }
        for j in 0..sorted_expected_coins.len() {
            if sorted_expected_coins[j].denom != sorted_result_coins[j].denom
                || sorted_expected_coins[j].amount != sorted_result_coins[j].amount
            {
                return false;
            }
        }
    }
    return true;
}

// The Test_Cases struct represents a group of test cases with a related name.
pub struct Test_Cases {
    case_name: String,
    cases: Vec<Test_Case>,
}

// The test function executes a single test case by calculating the result balances and comparing them to the expected balances using compare_balances.
fn test(test_case: Test_Case) {
    let result_balances = calculate_balance_changes(
        test_case.original_balances,
        test_case.definitions,
        test_case.multi_send_tx,
    );
    let expected_balances = test_case.result;
    match expected_balances {
        Ok(exp_balances) => {
            assert!(compare_balances(&exp_balances, &result_balances.unwrap()));
        }
        Err(exp_msg) => {
            assert_eq!(exp_msg, result_balances.unwrap_err());
        }
    }
}

#[test]
// The test_all function runs all of the test cases defined in the test_cases module, which is not shown here.
fn test_all() {
    let vec_test_cases: Vec<Test_Cases> = vec![
        Test_Cases {
            case_name: "one input, one output, one denom".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![Coin {
                        denom: "denom1".to_string(),
                        amount: 1000_000,
                    }],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 0.08,
                    commission_rate: 0.12,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                },
                result: Ok(vec![
                    Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    },
                    Balance {
                        address: "issuer_account_A".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 120,
                        }],
                    },
                    Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: -1200,
                        }],
                    },
                ]),
            }],
        },
        Test_Cases {
            case_name: "no issuer on sender or receiver".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![
                    Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000_000,
                        }],
                    },
                    Balance {
                        address: "account2".to_string(),
                        coins: vec![Coin {
                            denom: "denom2".to_string(),
                            amount: 1000_000,
                        }],
                    },
                ],
                definitions: vec![
                    DenomDefinition {
                        denom: "denom1".to_string(),
                        issuer: "issuer_account_A".to_string(),
                        burn_rate: 0.08,
                        commission_rate: 0.12,
                    },
                    DenomDefinition {
                        denom: "denom2".to_string(),
                        issuer: "issuer_account_A".to_string(),
                        burn_rate: 1.0,
                        commission_rate: 0.0,
                    },
                ],
                multi_send_tx: MultiSend {
                    inputs: vec![
                        Balance {
                            address: "account1".to_string(),
                            coins: vec![Coin {
                                denom: "denom1".to_string(),
                                amount: 1000,
                            }],
                        },
                        Balance {
                            address: "account2".to_string(),
                            coins: vec![Coin {
                                denom: "denom2".to_string(),
                                amount: 1000,
                            }],
                        },
                    ],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 1000,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 1000,
                            },
                        ],
                    }],
                },
                result: Ok(vec![
                    Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 1000,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 1000,
                            },
                        ],
                    },
                    Balance {
                        address: "issuer_account_A".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 120,
                        }],
                    },
                    Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: -1200,
                        }],
                    },
                    Balance {
                        address: "account2".to_string(),
                        coins: vec![Coin {
                            denom: "denom2".to_string(),
                            amount: -2000,
                        }],
                    },
                ]),
            }],
        },
        Test_Cases {
            case_name: "multi input, multi output, multi denom".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![
                    Balance {
                        address: "addr1".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 1000,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 2000,
                            },
                        ],
                    },
                    Balance {
                        address: "addr2".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 500,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: 3000,
                            },
                        ],
                    },
                ],
                definitions: vec![
                    DenomDefinition {
                        denom: "denom1".to_string(),
                        issuer: "addr1".to_string(),
                        burn_rate: 0.1,
                        commission_rate: 0.05,
                    },
                    DenomDefinition {
                        denom: "denom2".to_string(),
                        issuer: "addr1".to_string(),
                        burn_rate: 0.2,
                        commission_rate: 0.1,
                    },
                    DenomDefinition {
                        denom: "denom3".to_string(),
                        issuer: "addr2".to_string(),
                        burn_rate: 0.15,
                        commission_rate: 0.07,
                    },
                ],
                multi_send_tx: MultiSend {
                    inputs: vec![
                        Balance {
                            address: "addr1".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 30,
                                },
                                Coin {
                                    denom: "denom2".to_string(),
                                    amount: 50,
                                },
                            ],
                        },
                        Balance {
                            address: "addr2".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 20,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 100,
                                },
                            ],
                        },
                    ],
                    outputs: vec![
                        Balance {
                            address: "addr1".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 25,
                                },
                                Coin {
                                    denom: "denom2".to_string(),
                                    amount: 40,
                                },
                            ],
                        },
                        Balance {
                            address: "addr2".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 15,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 80,
                                },
                            ],
                        },
                        Balance {
                            address: "addr3".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 10,
                                },
                                Coin {
                                    denom: "denom2".to_string(),
                                    amount: 10,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 20,
                                },
                            ],
                        },
                    ],
                },
                result: Ok(vec![
                    Balance {
                        address: "addr1".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: -4,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: -10,
                            },
                        ],
                    },
                    Balance {
                        address: "addr2".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: -8,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: -20,
                            },
                        ],
                    },
                    Balance {
                        address: "addr3".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 10,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 10,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: 20,
                            },
                        ],
                    },
                ]),
            }],
        },
        Test_Cases {
            case_name: "zero input".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![Coin {
                        denom: "denom1".to_string(),
                        amount: 0,
                    }],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 210000.0,
                    commission_rate: 0.12,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 0,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 0,
                        }],
                    }],
                },
                result: Ok(vec![]),
            }],
        },
        Test_Cases {
            case_name: "input output same".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![
                    Balance {
                        address: "addr1".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 3000,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 2000,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: 2000,
                            },
                        ],
                    },
                    Balance {
                        address: "addr2".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 5000,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: 3000,
                            },
                        ],
                    },
                ],
                definitions: vec![
                    DenomDefinition {
                        denom: "denom1".to_string(),
                        issuer: "addr1".to_string(),
                        burn_rate: 0.1,
                        commission_rate: 0.05,
                    },
                    DenomDefinition {
                        denom: "denom2".to_string(),
                        issuer: "addr2".to_string(),
                        burn_rate: 0.2,
                        commission_rate: 0.1,
                    },
                    DenomDefinition {
                        denom: "denom3".to_string(),
                        issuer: "addr3".to_string(),
                        burn_rate: 0.15,
                        commission_rate: 0.07,
                    },
                ],
                multi_send_tx: MultiSend {
                    inputs: vec![
                        Balance {
                            address: "addr1".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom2".to_string(),
                                    amount: 1000,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 1100,
                                },
                            ],
                        },
                        Balance {
                            address: "addr2".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 1200,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 1500,
                                },
                            ],
                        },
                    ],
                    outputs: vec![
                        Balance {
                            address: "addr1".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom2".to_string(),
                                    amount: 1000,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 1100,
                                },
                            ],
                        },
                        Balance {
                            address: "addr2".to_string(),
                            coins: vec![
                                Coin {
                                    denom: "denom1".to_string(),
                                    amount: 1200,
                                },
                                Coin {
                                    denom: "denom3".to_string(),
                                    amount: 1500,
                                },
                            ],
                        },
                    ],
                },
                result: Ok(vec![
                    Balance {
                        address: "addr1".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: 60,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: -300,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: -242,
                            },
                        ],
                    },
                    Balance {
                        address: "addr2".to_string(),
                        coins: vec![
                            Coin {
                                denom: "denom1".to_string(),
                                amount: -180,
                            },
                            Coin {
                                denom: "denom2".to_string(),
                                amount: 100,
                            },
                            Coin {
                                denom: "denom3".to_string(),
                                amount: -330,
                            },
                        ],
                    },
                    Balance {
                        address: "addr3".to_string(),
                        coins: vec![Coin {
                            denom: "denom3".to_string(),
                            amount: 182,
                        }],
                    },
                ]),
            }],
        },
        Test_Cases {
            case_name: "input output mismatch".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![Coin {
                        denom: "denom1".to_string(),
                        amount: 1000_000,
                    }],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 0.0,
                    commission_rate: 0.0,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 350,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 450,
                        }],
                    }],
                },
                result: Err("notice that input and output does not match".to_string()),
            }],
        },
        Test_Cases {
            case_name: "min balance".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![Coin {
                        denom: "denom1".to_string(),
                        amount: 1200,
                    }],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 0.08,
                    commission_rate: 0.12,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                },
                result: Ok(vec![
                    Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    },
                    Balance {
                        address: "issuer_account_A".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 120,
                        }],
                    },
                    Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: -1200,
                        }],
                    },
                ]),
            }],
        },
        Test_Cases {
            case_name: "min balance - 1".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![Coin {
                        denom: "denom1".to_string(),
                        amount: 1199,
                    }],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 0.08,
                    commission_rate: 0.12,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 1000,
                        }],
                    }],
                },
                result: Err(
                    "notice that account1 does not have enough balance for denom1".to_string(),
                ),
            }],
        },
        Test_Cases {
            case_name: "not enough balance".to_string(),
            cases: vec![Test_Case {
                original_balances: vec![Balance {
                    address: "account1".to_string(),
                    coins: vec![],
                }],
                definitions: vec![DenomDefinition {
                    denom: "denom1".to_string(),
                    issuer: "issuer_account_A".to_string(),
                    burn_rate: 0.0,
                    commission_rate: 0.0,
                }],
                multi_send_tx: MultiSend {
                    inputs: vec![Balance {
                        address: "account1".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 350,
                        }],
                    }],
                    outputs: vec![Balance {
                        address: "account_recipient".to_string(),
                        coins: vec![Coin {
                            denom: "denom1".to_string(),
                            amount: 350,
                        }],
                    }],
                },
                result: Err(
                    "notice that account1 does not have enough balance for denom1".to_string(),
                ),
            }],
        },
    ];
    for test_cases in vec_test_cases {
        println!(
            "Test Case: {:?}, Count: {}",
            test_cases.case_name,
            test_cases.cases.len()
        );
        for test_case in test_cases.cases {
            test(test_case);
        }
    }
}
  