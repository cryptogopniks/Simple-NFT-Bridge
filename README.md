### Project Description

**Simple NFT Bridge** allows to send over IBC whitelisted NFT collections from any home network (directly or through common IBC connected network) to Neutron and vise versa

### Architecture

```mermaid
---
config:
  theme: dark
---
flowchart TD
  n1["Home Outpost<br>(Oraichain)"] --> n2["Retranslation Outpost<br>(Cosmos Hub)"]
  n2 --> n3["Hub<br>(Neutron)"]
  n3 --> n4["NFT Minter<br>(Neutron)"]
  n5["Home Outpost<br>(Cosmos Hub)"] --> n3
  n6["Home Outpost<br>(Stargaze)"] --> n3
```
