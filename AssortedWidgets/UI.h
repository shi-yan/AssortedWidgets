#pragma once

#include "Menubar.h"
#include "FontEngine.h"
#include "DefaultTheme.h"
#include "Menu.h"
#include "MenuBar.h"
#include "MouseEvent.h"
#include "MenuItemButton.h"
#include "MenuItemSeparator.h"
#include "MenuItemSubMenu.h"
#include "MenuItemToggleButton.h"
#include "Label.h"
#include "Button.h"
#include "MenuItemRadioButton.h"
#include "MenuItemRadioGroup.h"
#include "SelectionManager.h"
#include "DragManager.h"
#include "Dialog.h"
#include "FlowLayout.h"
#include "BorderLayout.h"
#include "GirdLayout.h"
#include "TextField.h"
#include "TypeActiveManager.h"
#include "Logo.h"
#include "ScrollBar.h"
#include "ScrollPanel.h"
#include "CheckButton.h"
#include "RadioButton.h"
#include "RadioGroup.h"
#include "ProgressBar.h"
#include "SlideBar.h"
#include "DropList.h"
#include "DropListManager.h"
#include "DialogManager.h"
#include "LabelNButtonTestDialog.h"
#include "CheckNRadioTestDialog.h"
#include "ProgressNSliderTestDialog.h"
#include "TextNDropTestDialog.h"
#include "FlowLayoutTestDialog.h"

#include "BorderLayoutTestDialog.h"
#include "GirdLayoutTestDialog.h"
#include "MultipleLayoutTestDialog.h"
#include "PanelTestDialog.h"
#include "AllInOneDialog.h"
#include "DialogTestDialog.h"

namespace AssortedWidgets
{
	class UI
	{
	private:
		Manager::SelectionManager selectionManager;
		int width;
		int height;
		int pressed;

		Widgets::Menu *menuFile;
		Widgets::Menu *menuEdit;
		Widgets::Menu *menuCreate;
		Widgets::Menu *menuModify;
		Widgets::Menu *menuSelection;
		Widgets::Menu *menuDisplay;
		Widgets::Menu *menuHelp;
		Widgets::MenuItemButton *menuItemFileOpen;
		Widgets::MenuItemButton *menuItemFileSave;
		Widgets::MenuItemButton *menuItemFileSaveAs;
		Widgets::MenuItemSubMenu *menuItemFileExport;
		Widgets::MenuItemSubMenu *menuItemFilePNG;
		Widgets::MenuItemButton *menuItemFilePNGNone;
		Widgets::MenuItemButton *menuItemFilePNGInterlaced;
		Widgets::MenuItemButton *menuItemFileJPEG;
		Widgets::MenuItemSubMenu *menuItemFileImport;
		Widgets::MenuItemButton *menuItemFile3DS;
		Widgets::MenuItemButton *menuItemFileOBJ;
		Widgets::MenuItemButton *menuItemFileSIA;
		Widgets::MenuItemSeparator *menuItemFileSeparator;
		Widgets::MenuItemButton *menuItemFileExit;
		Widgets::MenuItemButton *menuItemEditUndo;
		Widgets::MenuItemButton *menuItemEditRedo;
		Widgets::MenuItemToggleButton *menuItemEditShowConsole;
		Widgets::MenuItemButton *menuItemCreateCube;
		Widgets::MenuItemButton *menuItemCreateSphere;
		Widgets::MenuItemButton *menuItemCreatePlane;
		Widgets::MenuItemButton *menuItemCreateCylinder;
		Widgets::MenuItemButton *menuItemModifySplit;
		Widgets::MenuItemButton *menuItemModifyExtrude;
		Widgets::MenuItemButton *menuItemModifyDetach;
		Widgets::MenuItemButton *menuItemModifyWeld;
		Widgets::MenuItemButton *menuItemSelectionInvert;
		Widgets::MenuItemButton *menuItemSelectionFrame;
		Widgets::MenuItemButton *menuItemSelectionAll;
		Widgets::MenuItemButton *menuItemDisplayGhost;
		Widgets::MenuItemButton *menuItemDisplayWiredFrame;
		Widgets::MenuItemButton *menuItemDisplayFaced;
		Widgets::MenuItemButton *menuItemDisplaySmooth;
		Widgets::MenuItemButton *menuItemDisplayMaterial;
		Widgets::MenuItemRadioButton *menuItemDisplaySingle;
		Widgets::MenuItemRadioButton *menuItemDisplayTwo;
		Widgets::MenuItemRadioButton *menuItemDisplayThree;
		Widgets::MenuItemRadioButton *menuItemDisplayFour;
		Widgets::MenuItemRadioGroup *menuItemDisplayGroupTest;
		Widgets::MenuItemButton *menuItemHelpAbout;
		Widgets::MenuItemButton *menuItemHelpHelp;

		Widgets::Menu *menuAssortedWidgetsTest;
		Widgets::MenuItemButton *menuItemLabelNButtonTest;
		Test::LabelNButtonTestDialog *labelNButtonTestDialog;

		Widgets::MenuItemButton *menuItemCheckNRadioTest;
		Test::CheckNRadioTestDialog *checkNRadioTestDialog;

		Widgets::MenuItemButton *menuItemProgressNSliderTest;
		Test::ProgressNSliderTestDialog *progressNSliderTestDialog;

		
		Widgets::MenuItemButton *menuItemTextNDropTest;
		Test::TextNDropTestDialog *textNDropTestDialog;
	
		Widgets::MenuItemSubMenu *menuItemLayoutTest;
		Widgets::MenuItemButton *menuItemFlowTest;
		Test::FlowLayoutTestDialog *flowLayoutTestDialog;

		Widgets::MenuItemButton *menuItemBorderTest;
		Test::BorderLayoutTestDialog *borderLayoutTestDialog;

		Widgets::MenuItemButton *menuItemGirdTest;
		Test::GirdLayoutTestDialog *girdLayoutTestDialog;

		Widgets::MenuItemButton *menuItemMultipleTest;
		Test::MultipleLayoutTestDialog *multipleLayoutTestDialog;

		Widgets::MenuItemButton *menuItemPanelTest;
		Test::PanelTestDialog *panelTestDialog;

		Widgets::MenuItemButton *menuItemAllInOneTest;
		Test::AllInOneDialog *allInOneDialog;

		Widgets::MenuItemButton *menuItemDialogTest;
		Test::DialogTestDialog *dialogTestDialog;


		std::vector<Widgets::Component*> componentList;
		Widgets::Logo *logo;
		UI(void);
		void begin2D();
		void end2D();
	public:
		void paint();

		void importKeyDown(int keyCode,int modifier)
		{
			if(Manager::TypeActiveManager::getSingleton().isActive())
			{
				Manager::TypeActiveManager::getSingleton().onCharTyped(static_cast<char>(keyCode),modifier);
			}
		};

		void importKeyUp(int keyCode,int modifier)
		{

		};

		void importMousePress(unsigned int button,int x,int y)
		{
			pressed=true;
			Manager::DragManager::getSingleton().setCurrent(x,y);
			if(Manager::DropListManager::getSingleton().isDropped())
			{
				if(Manager::DropListManager::getSingleton().isIn(x,y))
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_PRESSED,x,y,0);
					Manager::DropListManager::getSingleton().importMousePressed(event);
				}
				else
				{
					Manager::DropListManager::getSingleton().shrinkBack();
				}
			}

			if(Manager::TypeActiveManager::getSingleton().isActive())
			{
				Manager::TypeActiveManager::getSingleton().disactive();
			}
			if(Widgets::MenuBar::getSingleton().isIn(x,y))
			{
				Event::MouseEvent event(0,Event::MouseEvent::MOUSE_PRESSED,x,y,button);
				Widgets::MenuBar::getSingleton().processMousePressed(event);
			}
			else
			{
				if(Widgets::MenuBar::getSingleton().isExpand())
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_PRESSED,x,y,button);
					Widgets::MenuBar::getSingleton().processMousePressed(event);
				}
			}

			Manager::DialogManager::getSingleton().importMousePressed(x,y);

			if(!componentList.empty())
			{
				//std::vector<Widgets::Element*> &hittedComponent=selectionManager.getHitComponents(x,y);
				//std::vector<Widgets::Element*>::iterator iter;
				std::vector<Widgets::Component*>::iterator iter;
				//for(iter=hittedComponent.begin();iter<hittedComponent.end();++iter)
				for(iter=componentList.begin();iter<componentList.end();++iter)
				{
					if((*iter)->isIn(x,y))
					{
						Event::MouseEvent event(0,Event::MouseEvent::MOUSE_PRESSED,x,y,button);
						(*iter)->processMousePressed(event);
						break;
					}
				}
			}
		};

		void importMouseRelease(unsigned int button,int x,int y)
		{
			Manager::DropListManager::getSingleton().setCurrent(x,y);
			if(pressed && Manager::DragManager::getSingleton().isOnDrag())
			{
				Manager::DragManager::getSingleton().dragEnd();
			};
			pressed=false;
			if(Widgets::MenuBar::getSingleton().isIn(x,y))
			{
				Event::MouseEvent event(0,Event::MouseEvent::MOUSE_RELEASED,x,y,button);
				Widgets::MenuBar::getSingleton().processMouseReleased(event);
			}
			else
			{
				if(Widgets::MenuBar::getSingleton().isExpand())
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_RELEASED,x,y,button);
					Widgets::MenuBar::getSingleton().processMouseReleased(event);
				}
			}

			Manager::DialogManager::getSingleton().importMouseReleased(x,y);

			if(!componentList.empty())
			{
//				std::vector<Widgets::Element*> &hittedComponent=selectionManager.getHitComponents(x,y);
//				std::vector<Widgets::Element*>::iterator iter;
				std::vector<Widgets::Component*>::iterator iter;
				//for(iter=hittedComponent.begin();iter<hittedComponent.end();++iter)
				for(iter=componentList.begin();iter<componentList.end();++iter)
				{
					if((*iter)->isIn(x,y))
					{
						Event::MouseEvent event(0,Event::MouseEvent::MOUSE_RELEASED,x,y,button);
						(*iter)->processMouseReleased(event);
						break;
					}
				}
			}
		};

		void init(int _width,int _height)
		{
			width=_width;
			height=_height;
			Theme::DefaultTheme *theme=new Theme::DefaultTheme(_width,_height);
			theme->setup();
			selectionManager.setup(width,height);
			Theme::ThemeEngine::getSingleton().setupTheme(theme);
			Widgets::MenuBar::getSingleton().init(width);

			menuFile=new Widgets::Menu("File");
			menuItemFileOpen=new Widgets::MenuItemButton("Open");
			menuItemFileSave=new Widgets::MenuItemButton("Save");
			menuItemFileSaveAs=new Widgets::MenuItemButton("Save As");
			menuItemFileExit=new Widgets::MenuItemButton("Exit");
			menuItemFileExport=new Widgets::MenuItemSubMenu("Export");
			menuItemFilePNG=new Widgets::MenuItemSubMenu("PNG Image");
			menuItemFilePNGNone=new Widgets::MenuItemButton("None");
			menuItemFilePNGInterlaced=new Widgets::MenuItemButton("Interlaced");
			menuItemFilePNG->addItem(menuItemFilePNGNone);
			menuItemFilePNG->addItem(menuItemFilePNGInterlaced);
			menuItemFileJPEG=new Widgets::MenuItemButton("JPEG Image");
			menuItemFileExport->addItem(menuItemFilePNG);
			menuItemFileExport->addItem(menuItemFileJPEG);
			menuItemFileImport=new Widgets::MenuItemSubMenu("Import");
			menuItemFile3DS=new Widgets::MenuItemButton("3DS Model");
			menuItemFileOBJ=new Widgets::MenuItemButton("OBJ Model");
			menuItemFileSIA=new Widgets::MenuItemButton("SIA Model");
			menuItemFileImport->addItem(menuItemFile3DS);
			menuItemFileImport->addItem(menuItemFileOBJ);
			menuItemFileImport->addItem(menuItemFileSIA);
			menuItemFileSeparator=new Widgets::MenuItemSeparator();
			menuFile->addItem(menuItemFileOpen);
			menuFile->addItem(menuItemFileSave);
			menuFile->addItem(menuItemFileSaveAs);
			menuFile->addItem(menuItemFileExport);
			menuFile->addItem(menuItemFileImport);
			menuFile->addItem(menuItemFileSeparator);
			menuFile->addItem(menuItemFileExit);

			Widgets::Component::MouseDelegate appExit;
			appExit.bind(this,&UI::appExit);
			menuItemFileExit->mousePressedHandlerList.push_back(appExit);

			menuEdit=new Widgets::Menu("Edit");
			menuItemEditUndo=new Widgets::MenuItemButton("Undo");
			menuItemEditRedo=new Widgets::MenuItemButton("Redo");
			menuItemEditShowConsole=new Widgets::MenuItemToggleButton("Show Console");
			menuEdit->addItem(menuItemEditUndo);
			menuEdit->addItem(menuItemEditRedo);
			menuEdit->addItem(menuItemEditShowConsole);

			menuCreate=new Widgets::Menu("Create");
			menuItemCreateCube=new Widgets::MenuItemButton("Cube");
			menuItemCreateSphere=new Widgets::MenuItemButton("Sphere");
			menuItemCreatePlane=new Widgets::MenuItemButton("Plane");
			menuItemCreateCylinder=new Widgets::MenuItemButton("Cylinder");
			menuCreate->addItem(menuItemCreateCube);
			menuCreate->addItem(menuItemCreateSphere);
			menuCreate->addItem(menuItemCreatePlane);
			menuCreate->addItem(menuItemCreateCylinder);
			
			menuModify=new Widgets::Menu("Modify");
			menuItemModifySplit=new Widgets::MenuItemButton("Split");
			menuItemModifyExtrude=new Widgets::MenuItemButton("Extrude");
			menuItemModifyDetach=new Widgets::MenuItemButton("Detach");
			menuItemModifyWeld=new Widgets::MenuItemButton("Weld");
			menuModify->addItem(menuItemModifySplit);
			menuModify->addItem(menuItemModifyExtrude);
			menuModify->addItem(menuItemModifyDetach);
			menuModify->addItem(menuItemModifyWeld);

			menuSelection=new Widgets::Menu("Selection");
			menuItemSelectionInvert=new Widgets::MenuItemButton("Invert");
			menuItemSelectionFrame=new Widgets::MenuItemButton("Frame");
			menuItemSelectionAll=new Widgets::MenuItemButton("All");
			menuSelection->addItem(menuItemSelectionInvert);
			menuSelection->addItem(menuItemSelectionFrame);
			menuSelection->addItem(menuItemSelectionAll);

			menuDisplay=new Widgets::Menu("Display");
			menuItemDisplayGhost=new Widgets::MenuItemButton("Ghost");
			menuItemDisplayWiredFrame=new Widgets::MenuItemButton("Wired Frame");
			menuItemDisplayFaced=new Widgets::MenuItemButton("Faced");
			menuItemDisplaySmooth=new Widgets::MenuItemButton("Smooth");
			menuItemDisplayMaterial=new Widgets::MenuItemButton("Material");
			menuItemDisplaySingle=new Widgets::MenuItemRadioButton("Single View");
			menuItemDisplayTwo=new Widgets::MenuItemRadioButton("Two Views");
			menuItemDisplayThree=new Widgets::MenuItemRadioButton("Three Views");
			menuItemDisplayFour=new Widgets::MenuItemRadioButton("Four Views");
			menuItemDisplayGroupTest=new Widgets::MenuItemRadioGroup();
			menuItemDisplayGroupTest->addItem(menuItemDisplaySingle);
			menuItemDisplayGroupTest->addItem(menuItemDisplayTwo);
			menuItemDisplayGroupTest->addItem(menuItemDisplayThree);
			menuItemDisplayGroupTest->addItem(menuItemDisplayFour);
			menuDisplay->addItem(menuItemDisplayGhost);
			menuDisplay->addItem(menuItemDisplayWiredFrame);
			menuDisplay->addItem(menuItemDisplayFaced);
			menuDisplay->addItem(menuItemDisplaySmooth);
			menuDisplay->addItem(menuItemDisplayGroupTest);
			menuDisplay->addItem(menuItemDisplayMaterial);

			menuHelp=new Widgets::Menu("Help");
			menuItemHelpAbout=new Widgets::MenuItemButton("About");
			menuItemHelpHelp=new Widgets::MenuItemButton("Help");
			menuHelp->addItem(menuItemHelpAbout);
			menuHelp->addItem(menuItemHelpHelp);

			menuAssortedWidgetsTest=new Widgets::Menu("Assorted Widgets Test");
			menuItemLabelNButtonTest=new Widgets::MenuItemButton("Label & Button Test");
			menuAssortedWidgetsTest->addItem(menuItemLabelNButtonTest);
			menuItemCheckNRadioTest=new Widgets::MenuItemButton("Check & Radio Test");
			menuAssortedWidgetsTest->addItem(menuItemCheckNRadioTest);
			menuItemProgressNSliderTest=new Widgets::MenuItemButton("Progress & Slider Test");
			menuAssortedWidgetsTest->addItem(menuItemProgressNSliderTest);
			menuItemTextNDropTest=new Widgets::MenuItemButton("TextField & DropList Test");
			menuAssortedWidgetsTest->addItem(menuItemTextNDropTest);
			

			menuItemLayoutTest=new Widgets::MenuItemSubMenu("Layout Test");
			menuItemFlowTest=new Widgets::MenuItemButton("FlowLayout Test");
			menuItemLayoutTest->addItem(menuItemFlowTest);
			menuItemBorderTest=new Widgets::MenuItemButton("BorderLayout Test");
			menuItemLayoutTest->addItem(menuItemBorderTest);
			menuItemGirdTest=new Widgets::MenuItemButton("GirdLayout Test");
			menuItemLayoutTest->addItem(menuItemGirdTest);
			menuItemMultipleTest=new Widgets::MenuItemButton("MultipleLayout Test");
			menuItemLayoutTest->addItem(menuItemMultipleTest);
			menuAssortedWidgetsTest->addItem(menuItemLayoutTest);

			menuItemPanelTest=new Widgets::MenuItemButton("Scroll Panel Test");
			menuAssortedWidgetsTest->addItem(menuItemPanelTest);
			menuItemAllInOneTest=new Widgets::MenuItemButton("All In One Test");
			menuAssortedWidgetsTest->addItem(menuItemAllInOneTest);
			menuItemDialogTest=new Widgets::MenuItemButton("Modal Dialog Test");
			menuAssortedWidgetsTest->addItem(menuItemDialogTest);

			Widgets::MenuBar::getSingleton().addMenu(menuFile);
			Widgets::MenuBar::getSingleton().addMenu(menuEdit);
			Widgets::MenuBar::getSingleton().addMenu(menuCreate);
			Widgets::MenuBar::getSingleton().addMenu(menuModify);
			Widgets::MenuBar::getSingleton().addMenu(menuSelection);
			Widgets::MenuBar::getSingleton().addMenu(menuDisplay);
			Widgets::MenuBar::getSingleton().addMenu(menuHelp);
			Widgets::MenuBar::getSingleton().addMenu(menuAssortedWidgetsTest);


			labelNButtonTestDialog=new Test::LabelNButtonTestDialog();
			Widgets::Component::MouseDelegate labelNButtonTest;
			labelNButtonTest.bind(this,&UI::labelNButtonTest);
			menuItemLabelNButtonTest->mouseReleasedHandlerList.push_back(labelNButtonTest);

			checkNRadioTestDialog=new Test::CheckNRadioTestDialog();
			Widgets::Component::MouseDelegate checkNRadioTest;
			checkNRadioTest.bind(this,&UI::checkNRadioTest);
			menuItemCheckNRadioTest->mouseReleasedHandlerList.push_back(checkNRadioTest);

			progressNSliderTestDialog=new Test::ProgressNSliderTestDialog();
			Widgets::Component::MouseDelegate progressNSliderTest;
			progressNSliderTest.bind(this,&UI::progressNSliderTest);
			menuItemProgressNSliderTest->mouseReleasedHandlerList.push_back(progressNSliderTest);

			textNDropTestDialog=new Test::TextNDropTestDialog();
			Widgets::Component::MouseDelegate textNDropTest;
			textNDropTest.bind(this,&UI::textNDropTest);
			menuItemTextNDropTest->mouseReleasedHandlerList.push_back(textNDropTest);

			flowLayoutTestDialog=new Test::FlowLayoutTestDialog();
			Widgets::Component::MouseDelegate flowLayoutTest;
			flowLayoutTest.bind(this,&UI::flowLayoutTest);
			menuItemFlowTest->mouseReleasedHandlerList.push_back(flowLayoutTest);
		


			borderLayoutTestDialog=new Test::BorderLayoutTestDialog();
			Widgets::Component::MouseDelegate borderLayoutTest;
			borderLayoutTest.bind(this,&UI::borderLayoutTest);
			menuItemBorderTest->mouseReleasedHandlerList.push_back(borderLayoutTest);

			girdLayoutTestDialog=new Test::GirdLayoutTestDialog();
			Widgets::Component::MouseDelegate girdLayoutTest;
			girdLayoutTest.bind(this,&UI::girdLayoutTest);
			menuItemGirdTest->mouseReleasedHandlerList.push_back(girdLayoutTest);

			multipleLayoutTestDialog=new Test::MultipleLayoutTestDialog();
			Widgets::Component::MouseDelegate multipleLayoutTest;
			multipleLayoutTest.bind(this,&UI::multipleLayoutTest);
			menuItemMultipleTest->mouseReleasedHandlerList.push_back(multipleLayoutTest);

		
			panelTestDialog=new Test::PanelTestDialog();
			Widgets::Component::MouseDelegate panelTest;
			panelTest.bind(this,&UI::panelTest);
			menuItemPanelTest->mouseReleasedHandlerList.push_back(panelTest);
		
			allInOneDialog=new Test::AllInOneDialog();
			Widgets::Component::MouseDelegate allInOneTest;
			allInOneTest.bind(this,&UI::allInOneTest);
			menuItemAllInOneTest->mouseReleasedHandlerList.push_back(allInOneTest);
		
			dialogTestDialog=new Test::DialogTestDialog();
			Widgets::Component::MouseDelegate dialogTest;
			dialogTest.bind(this,&UI::dialogTest);
			menuItemDialogTest->mouseReleasedHandlerList.push_back(dialogTest);

			logo=new Widgets::Logo();
			logo->position.x=width-logo->size.width-20;
			logo->position.y=height-logo->size.height-20;

		//	selectionManager.registerComponent(labelTest);
		//	selectionManager.registerComponent(buttonTest);

		};

		static UI &getSingleton()
		{
			static UI obj;
			return obj;
		};

		void dialogTest(const Event::MouseEvent &e)
		{
			if(dialogTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModalDialog(dialogTestDialog);
			}
			else
			{
				dialogTestDialog->Close();
			}
		};

		void allInOneTest(const Event::MouseEvent &e)
		{
			if(allInOneDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(allInOneDialog);
			}
			else
			{
				allInOneDialog->Close();
			}
		};

		void panelTest(const Event::MouseEvent &e)
		{
			if(panelTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(panelTestDialog);
			}
			else
			{
				panelTestDialog->Close();
			}
		};

		void multipleLayoutTest(const Event::MouseEvent &e)
		{
			if(multipleLayoutTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(multipleLayoutTestDialog);
			}
			else
			{
				multipleLayoutTestDialog->Close();
			}
		};

		void girdLayoutTest(const Event::MouseEvent &e)
		{
			if(girdLayoutTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(girdLayoutTestDialog);
			}
			else
			{
				girdLayoutTestDialog->Close();
			}
		};

		void borderLayoutTest(const Event::MouseEvent &e)
		{
			if(borderLayoutTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(borderLayoutTestDialog);
			}
			else
			{
				borderLayoutTestDialog->Close();
			}
		};

		void flowLayoutTest(const Event::MouseEvent &e)
		{
			if(flowLayoutTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(flowLayoutTestDialog);
			}
			else
			{
				flowLayoutTestDialog->Close();
			}
		};

		void textNDropTest(const Event::MouseEvent &e)
		{
			if(textNDropTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(textNDropTestDialog);
			}
			else
			{
				textNDropTestDialog->Close();
			}
		};

		void progressNSliderTest(const Event::MouseEvent &e)
		{
			if(progressNSliderTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(progressNSliderTestDialog);
			}
			else
			{
				progressNSliderTestDialog->Close();
			}
		};

		void checkNRadioTest(const Event::MouseEvent &e)
		{
			if(checkNRadioTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(checkNRadioTestDialog);
			}
			else
			{
				checkNRadioTestDialog->Close();
			}
		};

		void labelNButtonTest(const Event::MouseEvent &e)
		{
			if(labelNButtonTestDialog->getShowType()==Widgets::Dialog::None)
			{
				Manager::DialogManager::getSingleton().setModelessDialog(labelNButtonTestDialog);
			}
			else
			{
				labelNButtonTestDialog->Close();
			}
		};

		void appExit(const Event::MouseEvent &e)
		{
			exit(0);
		};

		void mouseMotion(int mx,int my)
		{
			if(pressed && Manager::DragManager::getSingleton().isOnDrag())
			{
				Manager::DragManager::getSingleton().processDrag(mx,my);
				return;
			};
			if(Manager::DropListManager::getSingleton().isDropped())
			{
				if(Manager::DropListManager::getSingleton().isIn(mx,my))
				{
					if(Manager::DropListManager::getSingleton().isHover)
					{
						Event::MouseEvent event(0,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
						Manager::DropListManager::getSingleton().importMouseMotion(event);
					}
					else
					{
						Event::MouseEvent event(0,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
						Manager::DropListManager::getSingleton().importMouseEntered(event);						
					}
				
				}
				else
				{
					if(Manager::DropListManager::getSingleton().isHover)
					{
						Event::MouseEvent event(0,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						Manager::DropListManager::getSingleton().importMouseExited(event);
					}
				}
			}
			if(Widgets::MenuBar::getSingleton().isIn(mx,my))
			{
				if(Widgets::MenuBar::getSingleton().isHover)
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
					Widgets::MenuBar::getSingleton().processMouseMoved(event);
				}
				else
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
					Widgets::MenuBar::getSingleton().processMouseEntered(event);
				}
			}
			else
			{
				if(Widgets::MenuBar::getSingleton().isHover)
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
					Widgets::MenuBar::getSingleton().processMouseExited(event);
				}
				if(Widgets::MenuBar::getSingleton().isExpand())
				{
					Event::MouseEvent event(0,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
					Widgets::MenuBar::getSingleton().processMouseMoved(event);					
				}
			}

			Manager::DialogManager::getSingleton().importMouseMotion(mx,my);

			if(!componentList.empty())
			{
//				std::vector<Widgets::Element*> &hittedComponent=selectionManager.getHitComponents(mx,my);
//				std::vector<Widgets::Element*>::iterator iter;
				std::vector<Widgets::Component*>::iterator iter;
				//for(iter=hittedComponent.begin();iter<hittedComponent.end();++iter)
				for(iter=componentList.begin();iter<componentList.end();++iter)
				{
					if((*iter)->isIn(mx,my))
					{
						if((*iter)->isHover)
						{
							Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_MOTION,mx,my,0);
							(*iter)->processMouseMoved(event);	
						}
						else
						{
							Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
							(*iter)->processMouseEntered(event);												
						}
					}
					else
					{
						if((*iter)->isHover)
						{
							Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
							(*iter)->processMouseExited(event);												
						}
					}
				}
			}
		};
	private:	
		~UI(void);
	};
}