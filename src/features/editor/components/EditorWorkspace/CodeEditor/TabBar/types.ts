export interface TabProps {
    id: number;
    name: string;
    isActive: boolean;
    showClose: boolean;
    filePath?: string;
}

export interface TabEvents {
    select: [tabId: number];
    rename: [tabId: number, newName: string];
    close: [tabId: number];
}
