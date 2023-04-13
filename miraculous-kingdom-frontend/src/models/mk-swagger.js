
/* eslint-disable @typescript-eslint/no-var-requires,import/no-extraneous-dependencies,@typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-call */
const path = require("path");
const https = require("https");
const { codegen } = require("swagger-axios-codegen");
const axios = require("axios");
const fs = require("fs");
const { execSync } = require("child_process");
const { program } = require("commander");

program
    .name("yarn codegen-data")
    .description("Generate TypeScript models and API client from Swagger")
    .option("-l, --local", "use Swagger Url from .env (.env.local) file")
    .option("-f, --file <file>", "specify swagger.json file path")
    .option("-d, --domain <domain>", "specify Swagger domain");

program.addHelpText(
    "after",
    `
Examples:
  $ yarn codegen -l
  $ yarn codegen -f ./schema.json
  $ yarn codegen -d`
);

program.parse();

const options = program.opts();

process.env.NODE_ENV = "development";
require("../../../config/env");

const agent = new https.Agent({
    rejectUnauthorized: false,
});

(async () => {
    if (options.local || options.domain) {
        const SCHEMA_URL = `${
            (options.domain && `https://${options.domain}`) ||
            process.env.REACT_APP_API_BASE ||
            ""
        }/api-docs/openapi.json`;
        console.log(`Using schema from: ${SCHEMA_URL}\n`);

        try {
            const response = await axios.get(SCHEMA_URL, { httpsAgent: agent });
            fs.writeFileSync(
                path.join(__dirname, "schema.json"),
                JSON.stringify(response.data)
            );
            execSync(
                `yarn prettier --write ${path.join(__dirname, "schema.json")}`
            );
        } catch (e) {
            console.error("Error: check schema URL");
            process.exit(1);
        }
    } else if (options.file) {
        try {
            console.log(`Using schema from: ${options.file}\n`);
            fs.copyFileSync(options.file, path.join(__dirname, "schema.json"));
            execSync(
                `yarn prettier --write ${path.join(__dirname, "schema.json")}`
            );
        } catch (e) {
            console.error("Error: check schema path");
            process.exit(1);
        }
    } else {
        program.help();
    }

    await codegen({
        methodNameMode: "path",
        useStaticMethod: true,
        // eslint-disable-next-line global-require
        source: require("./schema.json"),
        outputDir: __dirname,
        fileName: "mk-service.ts",
        // strictNullChecks: false,
        // modelMode: 'interface',
        modelMode: "class",
        generateValidationModel: true,
        useClassTransformer: true,
        // generateValidationModel: true,
        // useCustomerRequestInstance: true,
        useHeaderParameters: false,
    });
})();
