import { localConductorConfig, installation, sleep } from '../common'
import { constructLinkData, unixDate, now } from "./utils";

module.exports = (orchestrator) => {
    orchestrator.registerScenario("pagination testing ascending links", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)

        /// SIMPLE LINK TEST

        const numLinks = 35;
        let {out: linkData, timestamps} = constructLinkData(numLinks, 11111);
        for (let step = 0; step < numLinks; step++) {
            await alice_sc_happ.cells[0].call(
                "social_context",
                "add_link",
                {
                    linkExpression: linkData[step],
                    indexStrategy: {
                        type: "Full"
                    },
                }
            )
        }
        //Get all 35 messages and check that it works correctly
        const allLinks = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: unixDate, untilDate: now.toISOString(), limit: 35})
        console.log(allLinks);
        t.deepEqual(allLinks.length, 35);
        let last = undefined;
        for (let step = 0; step < allLinks.length; step ++) {
            if (last != undefined) {
                //@ts-ignore
                t.deepEqual(last.timestamp < allLinks[step].timestamp, true);
            }
            t.deepEqual(allLinks[step].data.target, `target-${numLinks-(step+1)}`)
            last = allLinks[step];
        }

        //Get first page, should be from unix timestamp -> now with limit of 10 and then use last result to get the next page
        const firstPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: unixDate, untilDate: now.toISOString(), limit: 10})
        console.log(firstPage)
        t.deepEqual(firstPage.length, 10);
        last = undefined;
        for (let step = 0; step < firstPage.length; step ++) {
            if (last != undefined) {
                //@ts-ignore
                t.deepEqual(last.timestamp < firstPage[step].timestamp, true);
            }
            t.deepEqual(firstPage[step].data.target, `target-${numLinks-(step+1)}`)
            last = firstPage[step];
        }

        const secondPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: firstPage[firstPage.length -1].timestamp, untilDate: now.toISOString(), limit: 10})
        console.log(secondPage, firstPage[firstPage.length -1].timestamp);
        t.deepEqual(secondPage.length, 10);
        last = undefined;
        for (let step = 0; step < secondPage.length; step ++) {
            if (last != undefined) {
                //@ts-ignore
                t.deepEqual(last.timestamp < secondPage[step].timestamp, true);
            }
            t.deepEqual(secondPage[step].data.target, `target-${numLinks-(step+10)}`)
            last = secondPage[step];
        }

        const thirdPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: secondPage[secondPage.length -1].timestamp, untilDate: now.toISOString(), limit: 10})
        console.log(thirdPage);
        t.deepEqual(thirdPage.length, 10);
        last = undefined;
        for (let step = 0; step < thirdPage.length; step ++) {
            if (last != undefined) {
                //@ts-ignore
                t.deepEqual(last.timestamp < thirdPage[step].timestamp, true);
            }
            t.deepEqual(thirdPage[step].data.target, `target-${numLinks-(step+19)}`)
            last = thirdPage[step];
        }

        const fourthPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: thirdPage[thirdPage.length -1].timestamp, untilDate: now.toISOString(), limit: 8})
        console.log(fourthPage);
        t.deepEqual(fourthPage.length, 8);
        last = undefined;
        for (let step = 0; step < fourthPage.length; step ++) {
            if (last != undefined) {
                //@ts-ignore
                t.deepEqual(last.timestamp < fourthPage[step].timestamp, true);
            }
            t.deepEqual(fourthPage[step].data.target, `target-${numLinks-(step+28)}`)
            last = fourthPage[step];
        }

        t.pass()
    })
}