module.exports = {
  client: {
    service: {
      name: 'general_service',
      // URL to the GraphQL API
      url: 'http://localhost:5050/graphql',
      localSchemaFile: "./schema.graphql"
    },
    // Files processed by the extension
    includes: [
      'src/**/*.vue',
      'src/**/*.js',
    ],
  },
}