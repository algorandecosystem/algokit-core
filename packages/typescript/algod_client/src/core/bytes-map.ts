// Auto-generated: map of model name -> array of camelCase field names that are bytes (Uint8Array) in domain
export const BYTES_FIELDS: Record<string, readonly string[]> = {
  AccountParticipation: ['selectionParticipationKey', 'voteParticipationKey', 'stateProofKey'],
  AssetParams: ['metadataHash', 'nameB64', 'unitNameB64', 'urlB64'],
  TealValue: ['bytes'],
  AvmKeyValue: ['key'],
  ApplicationParams: ['approvalProgram', 'clearStateProgram'],
  Box: ['name', 'value'],
  BoxDescriptor: ['name'],
  BoxReference: ['name'],
  Version: ['genesisHashB64'],
  StateProof: ['stateProof'],
  LightBlockHeaderProof: ['proof'],
  StateProofMessage: ['blockHeadersCommitment', 'votersCommitment'],
  ApplicationStateOperation: ['key'],
  SimulationTransactionExecTrace: ['approvalProgramHash', 'clearStateProgramHash', 'logicSigHash'],
  TransactionProof: ['proof', 'stibhash'],
  TransactionParams: ['genesisHash'],
}
