import { Logger } from 'winston'
import { createLogger } from '../logging'
import { MAX_RESULTS_PER_QUERY, QueryNodeApi } from './olympia-query-node/api'
import { MembershipConnectionFieldsFragment, MembershipFieldsFragment } from './olympia-query-node/generated/queries'

export type SnapshotManagerParams = {
  queryNodeApi: QueryNodeApi
}

export type MembershipsSnapshot = {
  members: MembershipFieldsFragment[]
}

export class SnapshotManager {
  name = 'Snapshot Manager'
  protected logger: Logger
  protected queryNodeApi: QueryNodeApi

  public constructor(params: SnapshotManagerParams) {
    this.queryNodeApi = params.queryNodeApi
    this.logger = createLogger(this.name)
  }

  private sortEntitiesByIds<T extends { id: string }>(entities: T[]): T[] {
    return entities.sort((a, b) => parseInt(a.id) - parseInt(b.id))
  }

  public async getMemberships(): Promise<MembershipFieldsFragment[]> {
    let lastCursor: string | undefined
    let currentPage: MembershipConnectionFieldsFragment
    let members: MembershipFieldsFragment[] = []
    do {
      this.logger.info(`Fetching a page of up to ${MAX_RESULTS_PER_QUERY} members...`)
      currentPage = await this.queryNodeApi.getMembershipsPage(lastCursor)
      members = members.concat(currentPage.edges.map((e) => e.node))
      this.logger.info(`Total ${members.length} members fetched`)
      lastCursor = currentPage.pageInfo.endCursor || undefined
    } while (currentPage.pageInfo.hasNextPage)
    this.logger.info('Finished members fetching')

    return this.sortEntitiesByIds(members)
  }

  public async createMembershipsSnapshot(): Promise<MembershipsSnapshot> {
    const members = await this.getMemberships()
    return {
      members,
    }
  }
}