import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainX } from "../target/types/onchain_x";


  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.onchainX ;

it('can send a new tweet', async () => {
    await program.rpc.sendTweet('Optimism', 'All is well?', {
        accounts: {
            // Accounts here...
        },
        signers: [
          	// Key pairs of signers here...
        ],
    });
});
