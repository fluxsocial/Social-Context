const now = new Date()
const unixDate = new Date("August 19, 1975 23:15:30").toISOString();

function constructTimestamps(num: number, diffMs: number): Date[] {
    let out: Date[] = [];
    let last = now;
    out.push(last);
    for (let step=0; step<num; step++) {
        let newTimestamp = new Date(last.getTime() - diffMs)
        console.log("Creating link with timestamp", newTimestamp);
        out.push(newTimestamp);
        last = newTimestamp;
    };
    return out
}

function constructGroupedTimestamps(num: number, groupings: number, diffMs: number, groupMSDiff: number) {
    let out: Date[] = [];
    let start = new Date(now.getTime() - (groupings * groupMSDiff));
    let linksPerGroup = num / groupings;
    let currentGrouping = 0;
    out.push(start);
    console.log("Starting from timestamp", start, linksPerGroup);
    for (let step=0; step<num; step++) {
        if (step % linksPerGroup === 0) {
            currentGrouping += 1;
        }
        let newTimestamp = new Date(start.getTime() + (diffMs * (step - (currentGrouping * linksPerGroup))) + (currentGrouping * groupMSDiff))
        console.log("Creating link with timestamp", newTimestamp, currentGrouping);
        out.push(newTimestamp);
    };
    return out
}

function constructLinkData(num: number, diffMs: number) {
    let out = [];
    let timestamps = constructTimestamps(num, diffMs);
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

function constructLongTimeLinkData(num: number, groupings: number, diffMs: number, groupMSDiff: number) {
    let out = [];
    let timestamps = constructGroupedTimestamps(num, groupings, diffMs, groupMSDiff);
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

export { constructTimestamps, constructGroupedTimestamps, constructLinkData, constructLongTimeLinkData, now, unixDate }