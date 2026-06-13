import React from 'react'
import EditExpenditureLink from './EditExpenditureLink'
import DeleteExpenditureLink from './DeleteExpenditureLink'
import { format } from 'd3-format'

export default function Expenditures ({ budgetId, expenditures, onDelete }) {
  return (
    <table className="expenditures table table-hover">
      <thead>
        <tr>
          <th>Date</th>
          <th>Amount</th>
          <th>Description</th>
          <th>Vendor</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {expenditures.map(expenditure => {
          return (
            <tr>
              <td clasName="expenditure-date">{expenditure.created_at}</td>
              <td className="expenditure-amount">{format("$,")(expenditure.amount / 100.0)}</td>
              <td className="expenditure-desc">{expenditure.description}</td>
              <td className="expenditure-vendor">{expenditure.vendor}</td>
              <td className="expenditure-actions">
                <EditExpenditureLink budgetId={budgetId} expenditure={expenditure} />
                <DeleteExpenditureLink
                  expenditure={expenditure}
                  onDelete={onDelete}
                />
              </td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}
