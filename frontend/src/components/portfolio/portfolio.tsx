import React from 'react';
import { Card } from '../ui/card';
import { useAnchorProgram } from '@/hooks/useAnchorProject';
import { fromDecimalsString } from '@/utils/reverseDecimals';
import Spinner from '../ui/spinner';

export const Portfolio: React.FC = () => {
    const { provider, userTokenAccountsQuery } = useAnchorProgram(); // Custom hook to fetch token accounts

    if (!provider.wallet?.publicKey) return <div>Connect your wallet...</div>;

    if (userTokenAccountsQuery.isLoading || userTokenAccountsQuery.isFetching) {
        return (
            <div className="py-20 px-4">
                <div className="max-w-4xl mx-auto">
                    <div className="flex flex-col items-center justify-center space-y-4">
                        <Spinner />
                        <span className="text-gray-400 text-lg">Loading your portfolio...</span>
                    </div>
                </div>
            </div>
        );
    }

    if (userTokenAccountsQuery.isError) {
        return (
          <div className="py-10 px-4 text-center">
                <p className="text-gray-400">Error loading your portfolio</p>
            </div>
        );
    }

    if (!userTokenAccountsQuery.data || userTokenAccountsQuery.data.length === 0) {
        return (
            <div className="py-10 px-4 text-center">
                <p className="text-gray-400">Your portfolio is empty.</p>
            </div>
        );
    }

    return (
        <div className="py-10 px-4">
        <div className="max-w-4xl mx-auto">
            <h1 className="text-3xl font-extrabold text-gray-300 mb-2 text-center">Your Owned Tokens</h1>
            <p className="text-lg text-gray-500 mb-8 text-center">A summary of all tokens you currently hold in your portfolio.</p>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
              {userTokenAccountsQuery.data.map((token, idx) =>
                token ? (
                  <Card
                    key={idx}
                    className="hover:cursor-pointer flex items-center gap-4 p-4 window-shadow rounded-xl shadow-md transition-all duration-300 ease-out transform hover:scale-105 hover:-translate-y-2 hover:shadow-2xl hover:border-indigo-400 border border-gray-400/30"
                  >
                    <img src={token.uri} alt={token.name + ' logo'} className="w-12 h-12 rounded-full object-cover transition-all duration-300 ease-out group-hover:scale-110" />
                    <div className="flex flex-col">
                      <span className="font-semibold text-lg">{token.name}</span>
                      <span className="text-gray-500">{token.symbol}</span>
                      <span className="text-sm text-gray-700">Supply: {fromDecimalsString(token.supply, token.decimals)}</span>
                    </div>
                  </Card>
                ) : null
              )}
            </div>
        </div>
        </div>
    );
    };
