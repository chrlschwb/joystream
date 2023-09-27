import { assert } from 'chai'
import Long from 'long'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { WorkerId, ForumPostId } from '@joystream/types/primitives'
import { RemarkMetadataAction } from '@joystream/metadata-protobuf'
import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { EventDetails } from '../../types'
import { Utils } from '../../utils'
import { BaseQueryNodeFixture } from '../../Fixture'

export type ValidaotrAccountInput = {
  memberId: ForumPostId
  validatorAccount: string
  asWorker?: WorkerId
}

export class VerifyValidatorAccountFixture extends BaseQueryNodeFixture {
  protected verifyValidator: ValidaotrAccountInput[]

  public constructor(api: Api, query: QueryNodeApi, verifyValidator: ValidaotrAccountInput[]) {
    super(api, query)
    this.verifyValidator = verifyValidator
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.verifyValidator.map((u) => {
      const metadata = Utils.metadataToBytes(RemarkMetadataAction, {
        verifyValidator: { memberId: Long.fromString(String(u.memberId)), validatorAccount: u.validatorAccount },
      })
      return u.asWorker
        ? this.api.tx.operationsWorkingGroupBeta.workerRemark(u.asWorker, metadata)
        : this.api.tx.operationsWorkingGroupBeta.leadRemark(metadata)
    })
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    if (this.api.findEvent(result, 'operationsWorkingGroupBeta', 'WorkerRemarked')) {
      return this.api.getEventDetails(result, 'operationsWorkingGroupBeta', 'WorkerRemarked')
    } else {
      return this.api.getEventDetails(result, 'operationsWorkingGroupBeta', 'LeadRemarked')
    }
  }

  async execute(): Promise<void> {}
}
