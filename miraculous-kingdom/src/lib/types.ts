export class TableRow {
    public name: string;
    public value: number | string;

    public constructor(data: {} = {} as any | undefined) {
        this.name = data["name"] as string;
        this.value = data["value"] as string;
    }
}

export class TableData {
    public name: string;
    public roll: boolean;
    public chng: boolean;
    public data: TableRow[];

    public constructor(data: {} = {} as any | undefined ) {
        this.name = data["name"] as string; 
        this.roll = data["roll"] as boolean;
        let chngStag: boolean = data["chng"];
        if ( this.roll && !chngStag) {
            throw new Error("A Table cannot have a roll associated with it that does not have the ability to change a mutable stat associated with it."); 
        }
        this.chng = chngStag;
        this.data = data["data"] as TableRow[];
    }
}
