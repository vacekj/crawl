# crawl

- crawls a blockchain for contract creations and ouputs bytecode to disk.
- uses erigon db directly, or an rpc in case erigon is not available

## next steps
- check all contracts for code from sourcify or etherscan
- identify known vulnerabilities
- check state values against the most common vulns to see if exploit is still possible

## symbolic execution
- get bytecode, automatically transform it into constraints (using llms?)
- define a constraints and run a solver (https://www.youtube.com/watch?v=VkSR9jz_C-0)

## endgame
- analyze bytecode directly using llms
- "given this bytecode and this state, develop an exploit that yields a net profit"
