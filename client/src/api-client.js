const baseUrl = 'localhost:3001'

export default createClient(baseUrl)

function createClient (baseUrl) {
  return {
    createExpenditure (expenditure) {
      console.log(JSON.stringify(expenditure))
      return fetch(`/api/v1/expenditures`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          amount: expenditure.amount,
          vendor: expenditure.vendor,
          description: expenditure.description,
          expense_category_id: expenditure.expenseCategoryId
        })
      })
        .then(resp => resp.json())
    },

    getExpenditure (expenditureId) {
      return fetch(`/api/v1/expenditures/${expenditureId}`)
        .then(resp => resp.json())
    },

    updateExpenditure (expenditure) {
      return fetch(`/api/v1/expenditures/${expenditure.id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          expenditure: {
            vendor: expenditure.vendor,
            amount: expenditure.amount,
            description: expenditure.description
          }
        })
      })
    },

    deleteExpenditure (expenditureId) {
      return fetch(`/api/v1/expenditures/${expenditureId}`, {
        method: 'DELETE',
      })
    },

    createCategory (category) {
      console.log(JSON.stringify(category))
      return fetch(`/api/v1/expense_categories`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          category: {
            name: category.name,
            amount: category.amount,
            budget_id: category.budgetId
          }
        })
      })
        .then(resp => resp.json())
    },

    updateCategory (category) {
      return fetch(`/api/v1/expense_categories/${category.id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          category: {
            name: category.name,
            amount: category.amount,
            budget_id: category.budgetId
          }
        })
      })
    },

    deleteCategory (categoryId) {
      return fetch(`/api/v1/expense_categories/${categoryId}`, {
        method: 'DELETE'
      })
    },

    getCategory (categoryId) {
      return fetch(`/api/v1/expense_categories/${categoryId}`)
        .then(resp => resp.json())
    },

    getBudget (budgetId) {
      return fetch(`/api/v1/budgets/${budgetId}`)
        .then(resp => resp.json())
    },

    getBudgets () {
      return fetch("/api/v1/budgets")
        .then(resp => resp.json())
    }
  }
}


