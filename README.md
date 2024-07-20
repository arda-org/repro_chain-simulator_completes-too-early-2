The issue:
- UserInShard0 calls ScInShard1 with a token
  - ScInShard1 calls a dummy endpoint of ScInShard0
  - In its callback, ScInShard1 sends back a token to UserInShard0
- At the moment `/simulator/generate-blocks-until-transaction-processed` stops generating blocks, UserInShard0 still has not received the token back, and we need to generate manually 6 more blocks in order for him to receive the token.

# How to reproduce

```
npm install
npm run build
npm run test
```
