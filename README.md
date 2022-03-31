# Phone Book

The phonebook contract is responsible for creating new games and for listing existing games.


## Instantiating the contract
To instantiate the contract you need to pass the following parameters to it: 

| Parameter    | Type      | Description                                                   |
|--------------|-----------|---------------------------------------------------------------|
| stamp_addr   | HumanAddr | The address to an instance of the stamping contract           |
| stamp_hash   | String    | The code hash for the stamping contract                       |
| game_code_id | u64       | The code id of the game contract for instantiating a new game |
| game_hash    | String    | The code hash for the game contract                           |
| jackpot_addr | HumanAddr | The address to an instance of the jackpot contract            |
| jackpot_hash | String    | The code hash of the jackpot contract                         |

## Handle Functions
| Function name       | Description                                                                                                        | Admin Only? | Clears list of games? |
|---------------------|--------------------------------------------------------------------------------------------------------------------|-------------|-----------------------|
| Refresh             | Refreshes the list of games, deleting games older than the maximum removal timeout                                 | No          | Maybe                 |
| CrateNewTable       | Creates a new game table                                                                                           | No          | No                    |
| UpdateRemoveTimeout | Modifies the timeout after which a game is considered "done" and can be safely removed from the list               | Yes         | Maybe                 |
| UpdateValidCodeId   | Modifies the code id and code hash for new games                                                                   | Yes         | Yes                   |
| UpdateStamper       | Modifies the contract address and code hash for the NFT stamping contract                                          | Yes         | Yes                   |
| UpdateJackpot       | Modifies the contract address and code hash for the Jackpot contract                                               | Yes         | Yes                   |
| PassTheHatOn        | Changes the address of the admin                                                                                   | Yes         | No                    |
| RegisteredCallback  | Callback function called by the game contract once created, this stores the contract in the list of game contracts | No          | No                    |

## Queries
| Query Name | Description                       |
|------------|-----------------------------------|
| GetList    | Returns a list of available games |