/// This test file tests the functioning of social context w/ time index enabled & signals disabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'
import { localConductorConfig, installation } from './common'

const orchestrator = new Orchestrator()

const now = new Date()
const unixDate = new Date("August 19, 1975 23:15:30").toISOString();

function constructTimestamps(num: number, diffMs: number): Date[] {
    let out: Date[] = [];
    let last = now;
    out.push(last);
    for (let step =0; step<num; step++) {
        let newTimestamp = new Date(last.getTime() - diffMs)
        console.log("Creating link with timestamp", newTimestamp);
        out.push(newTimestamp);
        last = newTimestamp;
    };
    return out
}

function constructLinkData(num: number, diff: number) {
    let out = [];
    let timestamps = constructTimestamps(num, diff);
    for (let step=0; step < num; step++) {
        let data = {
            data: {
                source: "subject", 
                target: `target-${step}`, 
                predicate: "predicate",
            },
            author: "test1", 
            timestamp: timestamps[step].toISOString(), 
            proof: {
                signature: "sig", 
                key: "key"
            } 
        }
        //@ts-ignore
        out.push(data)
    }
    return {out, timestamps}
}

orchestrator.registerScenario("pagination testing descending links", async (s, t) => {
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
    const allLinks = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: now.toISOString(), untilDate: unixDate, limit: 35})
    t.deepEqual(allLinks.length, 35);
    let last = undefined;
    for (let step = 0; step < allLinks.length; step ++) {
        if (last != undefined) {
            //@ts-ignore
            t.deepEqual(last.timestamp > allLinks[step].timestamp, true);
        }
        t.deepEqual(allLinks[step].data.target, `target-${step}`)
        last = allLinks[step];
    }

    //Get first page, should be from now -> unix timestamp with limit of 10 and then use last result to get the next page
    const firstPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: now.toISOString(), untilDate: unixDate, limit: 10})
    console.log(firstPage)
    t.deepEqual(firstPage.length, 10);
    last = undefined;
    for (let step = 0; step < firstPage.length; step ++) {
        if (last != undefined) {
            //@ts-ignore
            t.deepEqual(last.timestamp > firstPage[step].timestamp, true);
        }
        t.deepEqual(firstPage[step].data.target, `target-${step}`)
        last = firstPage[step];
    }

    const secondPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: firstPage[firstPage.length -1].timestamp, untilDate: unixDate, limit: 10})
    console.log(secondPage);
    t.deepEqual(secondPage.length, 10);
    last = undefined;
    for (let step = 0; step < secondPage.length; step ++) {
        if (last != undefined) {
            //@ts-ignore
            t.deepEqual(last.timestamp > secondPage[step].timestamp, true);
        }
        t.deepEqual(secondPage[step].data.target, `target-${step+9}`)
        last = secondPage[step];
    }

    const thirdPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: secondPage[secondPage.length -1].timestamp, untilDate: unixDate, limit: 10})
    console.log(thirdPage);
    t.deepEqual(thirdPage.length, 10);
    last = undefined;
    for (let step = 0; step < thirdPage.length; step ++) {
        if (last != undefined) {
            //@ts-ignore
            t.deepEqual(last.timestamp > thirdPage[step].timestamp, true);
        }
        t.deepEqual(thirdPage[step].data.target, `target-${step+18}`)
        last = thirdPage[step];
    }

    const fourthPage = await alice_sc_happ.cells[0].call("social_context", "get_links", {source: "subject", target: null, predicate: "predicate", fromDate: thirdPage[thirdPage.length -1].timestamp, untilDate: unixDate, limit: 8})
    console.log(fourthPage);
    t.deepEqual(fourthPage.length, 8);
    last = undefined;
    for (let step = 0; step < fourthPage.length; step ++) {
        if (last != undefined) {
            //@ts-ignore
            t.deepEqual(last.timestamp > fourthPage[step].timestamp, true);
        }
        t.deepEqual(fourthPage[step].data.target, `target-${step+27}`)
        last = fourthPage[step];
    }

    t.pass()
})

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


// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)