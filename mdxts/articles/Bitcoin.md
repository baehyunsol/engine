---
date: [2025, 8, 4]
tags: [bitcoin, blockchain]
preview: whatever
---

# Blockchain

[[small]]\* This article is about bitcoin's blockchain. There are many other more advanced blockchains, but I want to focus on the fundamentals[[/small]]

A blockchain is literally a chian of blocks. A block records transactions and blocks are chained in a time order.

## Block

A block consists of a header, transactions and some metadata. A header contains a hash of its previous block header, a hash of its transactions, timestamp, and PoW (Proof Of Work). PoW contains 2 fields: target and nonce.

`target` is a 256-bit number, and when you hash the header, the result must be less than equal to `target`. If you want to create a block, you have to brute-force `nonce` value until the hash becomes small enough. `target` value is adjusted every 2 weeks. If the players are creating too many blocks (more than a block in 10 minutes), `target` becomes smaller so that it's more difficult to create a block, and vice versa.

It's somewhat like git in that you cannot rewrite history. If you try to change something in an old block, the hash of the old block would change, which would invalid all the following blocks.

## Transactions

Let's say Alice wants to send 10 bitcoins to Bob. They broadcast "Alice sends 10 bitcoins to Bob" to a lot of nodes in the network. The message is signed with Alice's private key.

The message sits in a memory pool. The nodes in the network tries to create a new block with transactions in the memory pool. A node can choose arbitrary transactions, but they usually choose transactions with higher fees.

As I mentioned [above](#block), creating a block is a tedious process. When a node successfully creates a node, it broadcasts "hey folks, this is a new node that I've just created. Could you verify this?" If the new node is valid, the other nodes accept the new node, add the node to their chain, and start creating a new node from there.

## Preventing frauds

Unlike traditional financial systems, there's no central authority in a blockchain. Instead, each node verifies the transactions and shares the results.

- What if 2 different nodes create new (and valid) blocks at the same time?
  - There's no problem at all. The new blocks will propagate and there will be different branches in the blockchain. The nodes will try to create a new node within their branch, and the branches will keep growing. The branches will soon have different lengths, unless the entire blockchain is controlled by attackers. When the branches have different lengths, it's resolved. When a node sees 2 different branches, it'll always choose a longer one. So the unlucky branch will be discarded even though it's valid.
- What if Alice sends 10 bitcoins to Bob even though she doesn't have any bitcoins?
  - There's no problem at all. Each node verifies the transaction, and a node will easily find out that Alice is a liar and will reject the transaction. Nobody can lie unless they control more than 50% of the network.
- What if a malicious node creates a malicious block?
  - Again, there's no problem at all. The other nodes, which are likely to be honest, will reject the block. The honest nodes will work on an honest branch, which grows much faster than the malicious branch because more nodes are honest, and the malicious branch will be discarded.
- What if someone controls 51% of the nodes in the network?
  - First of all, the bitcoin blockchain is really huge. It's extremely difficult for an individual or a corporation to have such control.
  - Let's say you're really rich and somehow gain control of the 60% of the network. Having such control will give you a lot of bitcoins. As long as you're honest, people will trust the blockchain and you can use your bitcoins. If you mess up with the blockchain, nobody will use bitcoins and you're losing all your bitcoins. So, why would you mess up?

## Incentives

If you create a block, you can add a special transaction to the block, which gives a few coins to you (Subsidy). This gives you an incentive to verify the transactions and blocks. How many coins the special block creates is controlled by the protocol. It's reduced every 4 years, and will eventually become 0.

Also, a transaction can explicitly give a fee to the creator. This fee will motivate nodes to create blocks even thought there's no Subsidy.