// Mapped in tsconfig.json to get the exported function from my anchor_project/src
import { getAnchorProjectProgram, getAnchorProjectProgramId } from '@project/anchor';

import { Cluster, PublicKey } from '@solana/web3.js'
import { useMemo } from 'react'

import { useCluster } from '@/components/cluster/cluster-data-access'
import { useAnchorProvider } from '@/components/solana/use-anchor-provider'

// Queries
import { useMutation } from '@tanstack/react-query'
import { useTransactionToast } from '@/components/use-transaction-toast';
import { toast } from 'sonner'
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';

export function useLiquidityPool() {
    const { cluster } = useCluster()

    const transactionToast = useTransactionToast()

    const provider = useAnchorProvider()
    // This takes my program ID from the anchor project exported code
    const programId = useMemo(() => getAnchorProjectProgramId(cluster.network as Cluster), [cluster])
    const program = useMemo(() => getAnchorProjectProgram(provider, programId), [provider, programId])

    const create_liquidity_pool = useMutation({
        mutationFn: async ({
            baseTokenMint,
            quoteTokenMint,
        }: {
            baseTokenMint: PublicKey;
            quoteTokenMint: PublicKey;
        }) => {
            
            const [poolPda, _poolBump] = await PublicKey.findProgramAddress(
                [
                Buffer.from("liquidity_pool"),
                baseTokenMint.toBuffer(),
                quoteTokenMint.toBuffer(),
                ],
                program.programId
            );

            const [lpmintPda, _lpmintBump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("lp-mint"), 
                    poolPda.toBuffer(),
                ],
                program.programId
            );

            const [escrowTokenAAccountPda, escrowTokenABump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("escrow-a"), 
                    poolPda.toBuffer(),
                ],
                program.programId
            );

            const [escrowTokenBAccountPda, escrowTokenBBump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("escrow-b"), 
                    poolPda.toBuffer(),
                ],
                program.programId
            );

            return await program.methods
                .initializeLiquidityPool() 
                .accounts({ 
                    creator: provider.wallet.publicKey,
                    pool: poolPda,
                    lpMint: lpmintPda,
                    escrowTokenAAccount: escrowTokenAAccountPda,
                    escrowTokenBAccount: escrowTokenBAccountPda,
                    tokenAMint: baseTokenMint,
                    tokenBMint: quoteTokenMint,  
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                })
                .signers([provider.wallet.payer!])
                .instruction();
        },
        onSuccess: async (signature) => transactionToast(signature.toString()),
        onError: () => toast.error('Failed to initialize liquidity pool')
    });
    return { program, programId, provider, create_liquidity_pool}
}
