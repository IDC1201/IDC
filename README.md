# IDC Classic & IDC Mainnet Doc
IDC Mainnet 

```
terra1d4j9lsl453mkvtlg4ctw8y52rdkhafsaefug0hq0z06phczuvvvs7uq0vg
```


STAKING IDC Mainnet

```
terra195zzdqpan255lgatrufl6rt7szuc3wggu52urmul79mzd7d85gjq8ys7zr
```

Swap IDC Classic running on Columbus-5 Terra Classic Network

```
www.interchaindaocoin.com
```

IDC info finder smart contract

```
https://finder.terra.money/classic/address/terra1hjyl8ymsd9sn59kx733aka4rxfelf43pw6fjjf
```

Swap by Using CLI #
When offer_asset Is Native & IBC Token #
All transactions can be executed with the below command:
```json
terrad tx wasm execute <contract-address> <handle-msg> <coins>
```
contract-address: The pair contract address in a swap transaction
handle-msg: The method and parameters of the execution, which will be explained following lines
coins: Transaction execution fee

```json
{
  "swap": {
    "offer_asset": {
      "info": {
        "native_token": {
          "denom": "uluna"
        }
      },
      "amount": "10"
    },
    "to": "<Addr>"
  }
}
```
swap.offer_asset represents your source asset. It is mandatory to acknowledge the decimal of the token setting when entering the value of swap.offer_asset.amount for an accurate transaction you want. For instance, the decimal of LUNA is 6, and it implies the value 10 of swap.offer_asset.amount expresses ‘10 x 10^-6in the actual amount. That means you should multiply with the matching value,10^(decimal)`.

swap.to is the destination token address. You don’t need to enter the amount to swap into since Terraswap calculates the price algorithmically.

After filling them out, you may choose to change it into an inline string (not necessary if you can make it with multiline):
```json
'{"swap":{"offer_asset": {"info" : {"native_token": {"denom": "uluna"}},"amount": "10"},"to": "<Addr>",}}'
```
This is your handle-msg. The handle-msg can be used to complete the CLI command to swap tokens.

When offer_asset Is Contract-minted Token #
Swapping contract-minted token to native token is executed with the same logic as above. However, it requires a different handle-msg due to the token system’s difference and implementation. The message looks like:
```json
{
  "send": {
    "contract": "<Addr>",
    "amount": "10",
    "msg": Base64({
      "swap": {}
    })
  }
}
```
The method is a little bit tricky. Please keep your concentration on!

In the CLI,

contract-address: Enter token address
In the message:

send.contract: Enter pair address
send.amount: The amount of the origin token to swap from
send.msg.swap is an optional attribute, which is not required for basic swap executions.

Encode {"swap":{}} into base64 encoding. It should look something like: eyJzd2FwIjp7fX0=.

Enter the base64 encoded value for msg of the JSON:
```json
{
  "send": {
    "contract": "<Addr>",
    "amount": "10",
    "msg": "eyJzd2FwIjp7fX0="
  }
}
```
After then, you may proceed as the above

FOR Devoloper  :


EXAMPLE ON TERRA STATION


1.Open Terra station wallet

2.Contract

3.search Pair contract address

4.Execute

5.in Msg:

```json
{
  "swap": {
    "offer_asset": {
      "info": {
        "native_token": {
          "denom": "uusd"
        }
      },
      "amount": "10000000"
    },
    "to": ""
  }
}
```
Amount USTC :10 

swap.offer_asset represents your source asset. It is mandatory to acknowledge the decimal of the token setting when entering the value of swap.offer_asset.amount for an accurate transaction you want. For instance, the decimal of LUNA is 6, and it implies the value 10 of swap.offer_asset.amount expresses ‘10 x 10^-6in the actual amount. That means you should multiply with the matching value,10^(decimal)`.

swap.to is the destination token address. You don’t need to enter the amount to swap into since Terraswap calculates the price algorithmically.

After filling them out, you may choose to change it into an inline string (not necessary if you can make it with multiline):
