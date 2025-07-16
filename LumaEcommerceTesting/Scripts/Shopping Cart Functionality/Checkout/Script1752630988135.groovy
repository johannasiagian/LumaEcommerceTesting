import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://magento.softwaretestingboard.com/')

WebUI.click(findTestObject('Object Repository/Page_Home Page/a_Sign In'))

WebUI.setText(findTestObject('Object Repository/Page_Customer Login/input_Email_loginusername'), 'parto@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Customer Login/input_Password_loginpassword'), 'MNMCI3RQmCCEYTdVLX0vYg==')

WebUI.click(findTestObject('Object Repository/Page_Customer Login/button_Sign In'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/a_My Cart                    3             _2914b4'))

WebUI.click(findTestObject('Object Repository/Page_Home Page/button_Proceed to Checkout'))

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Company_company'), 'parto')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Street Address Line 1_street0'), 'alam')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Street Address Line 2_street1'), '123')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Street Address Line 3_street2'), '333')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_City_city'), 'Jakarta')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Checkout/select_Afghanistanland IslandsAlbaniaAlgeri_87ca51'), 
    'ID', true)

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_StateProvince_region'), 'Jakarta')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_ZipPostal Code_postcode'), '12345')

WebUI.setText(findTestObject('Object Repository/Page_Checkout/input_Phone Number_telephone'), '0812345678')

WebUI.click(findTestObject('Object Repository/Page_Checkout/button_Next'))

WebUI.click(findTestObject('Object Repository/Page_Checkout/button_Place Order'))

WebUI.closeBrowser()

