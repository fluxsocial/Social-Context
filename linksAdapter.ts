import type { Expression, LinksAdapter, NewLinksObserver, HolochainLanguageDelegate, LanguageContext, LinkQuery } from "@perspect3vism/ad4m";
import type { DID } from "@perspect3vism/ad4m/lib/DID";
import { DNA_NICK } from "./dna";

export class JuntoSocialContextLinkAdapter implements LinksAdapter {
  socialContextDna: HolochainLanguageDelegate;
  linkCallback?: NewLinksObserver

  constructor(context: LanguageContext) {
    //@ts-ignore
    this.socialContextDna = context.Holochain as HolochainLanguageDelegate;
  }

  writable(): boolean {
    return true;
  }

  public(): boolean {
    return false;
  }

  async others(): Promise<DID[]> {
    return []
  }

  async addActiveAgentLink(hcDna: HolochainLanguageDelegate): Promise<any> {
    if (hcDna == undefined) {
      //@ts-ignore
      return await this.call(
        DNA_NICK,
        "social_context",
        "add_active_agent_link",
        null
      );
    } else {
      return await hcDna.call(
        DNA_NICK,
        "social_context",
        "add_active_agent_link",
        null
      );
    }
  }

  async addLink(link: Expression): Promise<void> {
    const data = prepareExpressionLink(link);
    const input = {
      linkExpression: data,
      indexStrategy: {
          type: "FullWithWildCard"
      },
    }
    await this.socialContextDna.call(DNA_NICK, "social_context", "add_link", input);
  }

  async updateLink(
    oldLinkExpression: Expression,
    newLinkExpression: Expression
  ): Promise<void> {
    const source_link = prepareExpressionLink(oldLinkExpression);
    const target_link = prepareExpressionLink(newLinkExpression);
    const input = {
      source: source_link,
      target: target_link,
      indexStrategy: {
        type: "FullWithWildCard"
      },
    };
    await this.socialContextDna.call(
      DNA_NICK,
      "social_context",
      "update_link",
      input
    );
  }

  async removeLink(link: Expression): Promise<void> {
    const data = prepareExpressionLink(link);
    await this.socialContextDna.call(
      DNA_NICK,
      "social_context",
      "remove_link",
      data
    );
  }

  async getLinks(query: LinkQuery): Promise<Expression[]> {
    const link_query = Object.assign(query);
    if (link_query.source == undefined) {
      link_query.source = null;
    }
    if (link_query.target == undefined) {
      link_query.target = null;
    }
    if (link_query.predicate == undefined) {
      link_query.predicate = null;
    }
    if (link_query.fromDate) {
      link_query.fromDate = link_query.fromDate.toISOString();
    }
    if (link_query.untilDate) {
      link_query.untilDate = link_query.untilDate.toISOString();
    }
    const links = await this.socialContextDna.call(
      DNA_NICK,
      "social_context",
      "get_links",
      link_query
    );
    //console.debug("Holchain Social Context: Got Links", links);

    return links;
  }

  addCallback(callback: NewLinksObserver): number {
    this.linkCallback = callback;
    return 1;
  }

  handleHolochainSignal(signal: any): void {
    if (this.linkCallback) {
      this.linkCallback([signal.data.payload], []);
    }
  }
}

function prepareExpressionLink(link: Expression): object {
  const data = Object.assign(link);
  if (data.data.source == "") {
    data.data.source = null;
  }
  if (data.data.target == "") {
    data.data.target = null;
  }
  if (data.data.predicate == "") {
    data.data.predicate = null;
  }
  return data;
}
