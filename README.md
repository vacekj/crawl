# crawl

- crawls a blockchain for contract creations and ouputs bytecode to disk.
- uses erigon db directly, or an rpc in case erigon is not available

## next steps
- check all contracts for code from sourcify
- identify known vulnerabilities
- check state values against the most common vulns to see if exploit is still possible

endgame: analyze bytecode directly using gpt