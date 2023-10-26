import { FlowProps } from '../../Flow'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'

import { extendDebug } from '../../Debugger'
import { FixtureRunner } from '../../Fixture'
import { VerifyValidatorProfileFixture } from '../../fixtures/membership/VerifyValidatorAccountFixture'

export default async function validatorProfile({ api, query }: FlowProps): Promise<void> {
  const debug = extendDebug('flow:validator-account-update')
  debug('Started')
  api.enableDebugTxLogs()

  const VerifyValidator = [
    {
      memberId: '1',
      isVerifiedValidator: false,
    },
    {
      memberId: '2',
      isVerifiedValidator: false,
    },
    {
      memberId: '27',
      isVerifiedValidator: true,
    },
  ]

  const verifyAccountFixture = new VerifyValidatorProfileFixture(api, query, VerifyValidator)
  const remarkModerateRunner = new FixtureRunner(verifyAccountFixture)
  await remarkModerateRunner.run()
  await remarkModerateRunner.runQueryNodeChecks()

  debug('Done')
}