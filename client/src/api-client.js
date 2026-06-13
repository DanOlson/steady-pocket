export default createClient()

function request (url, options) {
  return fetch(url, options)
    .then(resp => {
      if (!resp.ok) {
        throw new Error(`${options?.method || 'GET'} ${url} failed: ${resp.status}`)
      }
      return resp
    })
}

function requestJson (url, options) {
  return request(url, options).then(resp => resp.json())
}

function jsonBody (method, body) {
  return {
    method,
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
  }
}

function createClient () {
  return {
    createExpenditure (expenditure) {
      return requestJson('/api/v1/expenditures', jsonBody('POST', {
        amount: expenditure.amount,
        vendor: expenditure.vendor,
        description: expenditure.description,
        budget_id: expenditure.budgetId,
        expense_category_id: expenditure.expenseCategoryId
      }))
    },

    getExpenditure (expenditureId) {
      return requestJson(`/api/v1/expenditures/${expenditureId}`)
    },

    updateExpenditure (expenditure) {
      return request(`/api/v1/expenditures/${expenditure.id}`, jsonBody('PATCH', {
        expenditure: {
          vendor: expenditure.vendor,
          amount: expenditure.amount,
          description: expenditure.description,
          expense_category_id: expenditure.expenseCategoryId
        }
      }))
    },

    deleteExpenditure (expenditureId) {
      return request(`/api/v1/expenditures/${expenditureId}`, { method: 'DELETE' })
    },

    createCategory (category) {
      return requestJson('/api/v1/expense_categories', jsonBody('POST', {
        category: {
          name: category.name,
          amount: category.amount,
          budget_id: category.budgetId
        }
      }))
    },

    updateCategory (category) {
      return request(`/api/v1/expense_categories/${category.id}`, jsonBody('PATCH', {
        category: {
          name: category.name,
          amount: category.amount,
          budget_id: category.budgetId
        }
      }))
    },

    deleteCategory (categoryId) {
      return request(`/api/v1/expense_categories/${categoryId}`, { method: 'DELETE' })
    },

    getCategory (categoryId) {
      return requestJson(`/api/v1/expense_categories/${categoryId}`)
    },

    getBudget (budgetId) {
      return requestJson(`/api/v1/budgets/${budgetId}`)
    },

    getBudgets () {
      return requestJson('/api/v1/budgets')
    }
  }
}
