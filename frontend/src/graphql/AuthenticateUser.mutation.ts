import gql from 'graphql-tag'

export const AUTHENTICATE_USER = gql`
    mutation authenticateUser($name: String!, $password: String!) {
        authenticateUser(input: {
            name: $name
            password: $password
        }) {
            id
            name
            authorization {
                level
            }
        }
    }
`