export interface Clock {
    id: string;
    city_name: string;
    timezone: string;
    order: number;
    time?: string;
    meridian?: string;
}
