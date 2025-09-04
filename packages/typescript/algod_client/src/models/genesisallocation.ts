export type GenesisAllocation = {
  addr: string;
  comment: string;
  state: { algo: bigint; onl: number; sel?: string; stprf?: string; vote?: string; voteKD?: bigint; voteFst?: bigint; voteLst?: bigint };
};
