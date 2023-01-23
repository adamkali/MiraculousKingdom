import { TableRow, TableData } from './types';

export const DIRECTORY = "./MiraculousKingdom";

export const CLASSES = {
        "001": "Cultural Aficianado",
        "002": "Military Advisor",
    };

export const MIGHTTABLE = new TableData({
    "name": "Might",
    "roll": true,
    "chng": true, 
    "data": [
        {
            "name": "Military Might",
            "value": 2
        },
        {
            "name": "Cultural Might",
            "value": 2
        },
        {
            "name": "Scientific Might",
            "value": 2
        },
        {
            "name": "Religious Might",
            "value": 2
        },
        {
            "name": "Diplomatic Might",
            "value": 2
        },
        {
            "name": "Criminal Might",
            "value": 2
        },
    ] as TableRow[]
});

export const TOKENTABLE = new TableData({
    "name": "Token",
    "roll": false,
    "chng": true,
    "data": [
        {
            "name": "Soldier Token",
            "value": 0,
        },
        {
            "name": "Art Token",
            "value": 0,
        },
        {
            "name": "Reason Token",
            "value": 0,
        },
        {
            "name": "Faith Token",
            "value": 0,
        },
        {
            "name": "Treaty Token",
            "value": 0,
        },
        {
            "name": "Crime Token",
            "value": 0,
        },
    ] as TableRow[]
}) 

