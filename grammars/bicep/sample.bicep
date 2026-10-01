// A sample.
targetScope = 'resourceGroup'

@description('The name of the storage account.')
@minLength(3)
param name string

param location string = resourceGroup().location
param limit int = 10
param enabled bool = true

var tags = {
  kind: 'sample'
  owner: 'nobody'
}

var names = [for i in range(0, limit): '${name}-${i}']

resource storage 'Microsoft.Storage/storageAccounts@2023-01-01' = {
  name: name
  location: location
  sku: {
    name: 'Standard_LRS'
  }
  kind: 'StorageV2'
  tags: tags
  properties: {
    supportsHttpsTrafficOnly: enabled
  }
}

resource containers 'Microsoft.Storage/storageAccounts/blobServices/containers@2023-01-01' = [for n in names: if (enabled) {
  name: '${storage.name}/default/${n}'
}]

module network './network.bicep' = {
  name: 'network'
  params: {
    location: location
  }
}

output id string = storage.id
output count int = length(names)
