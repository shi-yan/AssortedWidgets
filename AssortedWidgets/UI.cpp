#include <GL/gl.h>
#include <GL/glu.h>
#include "UI.h"

namespace AssortedWidgets
{
	UI::UI(void)
	{
	}

	void UI::begin2D()
	{
		glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
		glLoadIdentity();
		glMatrixMode(GL_PROJECTION);
		glPushMatrix();
		glLoadIdentity();
		gluOrtho2D(0,width,height,0);
		glMatrixMode(GL_MODELVIEW);
	}

	void UI::paint()
	{
		begin2D();
		glColor3ub(255,255,255);
		logo->paint();
		std::vector<Widgets::Component*>::iterator iter;
		for(iter=componentList.begin();iter<componentList.end();++iter)
		{
			(*iter)->paint();
		}
		Manager::DialogManager::getSingleton().paint();
		if(Manager::DropListManager::getSingleton().isDropped())
		{
			Manager::DropListManager::getSingleton().paint();
		}
		Widgets::MenuBar::getSingleton().paint();
		end2D();
	}

	void UI::end2D()
	{
		glMatrixMode(GL_PROJECTION);
		glPopMatrix();
		glMatrixMode(GL_MODELVIEW);
	}

	UI::~UI(void)
	{
		delete logo;
		componentList.clear();

		delete menuFile;
		delete menuEdit;
		delete menuCreate;
		delete menuModify;
		delete menuSelection;
		delete menuDisplay;
		delete menuHelp;

		delete menuItemFileOpen;
		delete menuItemFileSave;
		delete menuItemFileSaveAs;
		delete menuItemFileExport;
		delete menuItemFilePNG;
		delete menuItemFilePNGNone;
		delete menuItemFilePNGInterlaced;
		delete menuItemFileJPEG;
		delete menuItemFileImport;
		delete menuItemFile3DS;
		delete menuItemFileOBJ;
		delete menuItemFileSIA;
		delete menuItemFileSeparator;
		delete menuItemFileExit;
		delete menuItemEditUndo;
		delete menuItemEditRedo;
		delete menuItemEditShowConsole;
		delete menuItemCreateCube;
		delete menuItemCreateSphere;
		delete menuItemCreatePlane;
		delete menuItemCreateCylinder;
		delete menuItemModifySplit;
		delete menuItemModifyExtrude;
		delete menuItemModifyDetach;
		delete menuItemModifyWeld;
		delete menuItemSelectionInvert;
		delete menuItemSelectionFrame;
		delete menuItemSelectionAll;
		delete menuItemDisplayGhost;
		delete menuItemDisplayWiredFrame;
		delete menuItemDisplayFaced;
		delete menuItemDisplaySmooth;
		delete menuItemDisplayMaterial;
		delete menuItemDisplaySingle;
		delete menuItemDisplayTwo;
		delete menuItemDisplayThree;
		delete menuItemDisplayFour;
		delete menuItemDisplayGroupTest;
		delete menuItemHelpAbout;
		delete menuItemHelpHelp;

		delete menuAssortedWidgetsTest;
		delete menuItemLabelNButtonTest;
		delete labelNButtonTestDialog;
		delete menuItemCheckNRadioTest;
		delete checkNRadioTestDialog;
		delete menuItemProgressNSliderTest;
		delete progressNSliderTestDialog;

		delete menuItemTextNDropTest;
		delete textNDropTestDialog;

		delete menuItemLayoutTest;
		delete menuItemFlowTest;
		delete flowLayoutTestDialog;


		delete menuItemBorderTest;
		delete borderLayoutTestDialog;

		delete menuItemGirdTest;
		delete girdLayoutTestDialog;

		delete menuItemMultipleTest;
		delete multipleLayoutTestDialog;

		delete menuItemPanelTest;
		delete panelTestDialog;

		delete menuItemAllInOneTest;
		delete allInOneDialog;

		delete menuItemDialogTest;
		delete dialogTestDialog;
	}
}
