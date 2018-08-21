/*
 * Hedgewars, a free turn based strategy game
 * Copyright (c) 2004-2015 Andrey Korotaev <unC0Rr@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

#include <QDialog>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QPushButton>
#include <QLineEdit>
#include <QLabel>
#include <QDebug>

#include "seedprompt.h"

SeedPrompt::SeedPrompt(QWidget* parent, const QString & seed, bool editable) : QDialog(parent)
{
    setModal(true);
    setWindowFlags(Qt::Sheet);
    setWindowModality(Qt::WindowModal);
    //: Refers to the "random seed"; the source of randomness in the game
    setWindowTitle(tr("Seed"));
    setMinimumSize(360, 160);
    resize(360, 160);
    setSizePolicy(QSizePolicy::Minimum, QSizePolicy::Minimum);

    // Layout
    QVBoxLayout * dialogLayout = new QVBoxLayout(this);

    // Label
    QLabel * label = new QLabel(tr("The map seed is the basis for all random values generated by the game."));
    label->setWordWrap(true);
    dialogLayout->addWidget(label, 0);

    // Input box
    editBox = new QLineEdit();
    editBox->setText(seed);
    editBox->setReadOnly(!editable);
    editBox->setStyleSheet("QLineEdit { padding: 3px; }");
    dialogLayout->addWidget(editBox, 1);

    dialogLayout->addStretch(1);

    // Buttons
    QHBoxLayout * buttonLayout = new QHBoxLayout();
    buttonLayout->addStretch(1);
    dialogLayout->addLayout(buttonLayout);
    if (editable)
    {
        QPushButton * btnCancel = new QPushButton(tr("Cancel"));
        QPushButton * btnOkay = new QPushButton(tr("Set seed"));
        connect(btnCancel, SIGNAL(clicked()), this, SLOT(reject()));
        connect(btnOkay, SIGNAL(clicked()), this, SLOT(accept()));
#ifdef Q_OS_MAC
        buttonLayout->addWidget(btnCancel);
        buttonLayout->addWidget(btnOkay);
#else
        buttonLayout->addWidget(btnOkay);
        buttonLayout->addWidget(btnCancel);
#endif
        btnOkay->setDefault(true);
    }
    else
    {
        QPushButton * btnClose = new QPushButton(tr("Close"));
        connect(btnClose, SIGNAL(clicked()), this, SLOT(reject()));
        buttonLayout->addWidget(btnClose);
        btnClose->setDefault(true);
    }

    setStyleSheet("QPushButton { padding: 5px; }");

    connect(this, SIGNAL(accepted()), this, SLOT(setSeed()));
}

void SeedPrompt::setSeed()
{
    emit seedSelected(editBox->text());
}
