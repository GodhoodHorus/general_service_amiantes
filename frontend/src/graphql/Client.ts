import gql from 'graphql-tag'

export const CREATE_CLIENT = gql`
    
    mutation createClient($name: String!, $streetNumber: Int!, $street: String!) {
        createClient(input: {
            name: $name
            address: {
                streetNumber: $streetNumber,
                street: $street,
            }
            interlocutors: null
        }) {
            id
            name
            address {
                street
                streetNumber
            }
            createdAt
            editedAt
        }
    }
`

export const  GET_CLIENTS = gql`
    query clients($offset: Int!) {
        clients(offset: $offset) {
            id
            name
            address {
                street
                streetNumber
            }
            createdAt
            editedAt
        }
    }
`