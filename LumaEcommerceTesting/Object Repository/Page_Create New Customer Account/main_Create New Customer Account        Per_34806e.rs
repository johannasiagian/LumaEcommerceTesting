<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>main_Create New Customer Account        Per_34806e</name>
   <tag></tag>
   <elementGuidId>ad80e942-5d69-4f5b-ab78-3197ca637c68</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//main[@id='maincontent']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#maincontent</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Create New Customer Account Personalized recommendations Personal Information Fi&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>main</value>
      <webElementGuid>7f63703a-32cb-450b-9fa8-9dfeab2cab8b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>maincontent</value>
      <webElementGuid>1404bbc7-ec45-4c66-9412-84dff126c5cd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page-main</value>
      <webElementGuid>a692b61c-9f60-4f2d-a6d9-bd4dc23f8273</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

    
        Create New Customer Account    
    Personalized recommendations


    

    



    
        window.authenticationPopup = {&quot;autocomplete&quot;:&quot;off&quot;,&quot;customerRegisterUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/create\/&quot;,&quot;customerForgotPasswordUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/forgotpassword\/&quot;,&quot;baseUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/&quot;};
    
    



    







        
        Personal Information
        
        
        
            
            First Name
            
                
            
        
            
            Last Name
            
                
            
        
    
            
                
                
                    
        
        Sign-in Information
        
            Email
            
                
            
        
        
            Password
            
                Minimum of different classes of characters in password is 3. Classes of characters: Lower Case, Upper Case, Digits, Special Characters.
                
                    
                        Password Strength:
                        Very Strong
                    
                
            

        
        
            Confirm Password
            
                Please enter the same value again.
            
        
            
    
        
            Create an Account
        
        
            Back
        
    


require([
    'jquery',
    'mage/mage'
], function($){

    var dataForm = $('#form-validate');
    var ignore = null;

    dataForm.mage('validation', {
            ignore: ignore ? ':hidden:not(' + ignore + ')' : ':hidden'
        }).find('input:text').attr('autocomplete', 'off');
});



</value>
      <webElementGuid>bbd45e57-42f8-4ec6-93c6-919cdf7afa06</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;maincontent&quot;)</value>
      <webElementGuid>1068829f-01f6-489f-b7a6-e7198402492c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//main[@id='maincontent']</value>
      <webElementGuid>d5ecf7ed-4a03-4279-8ffe-b9f19a13e7bd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Create an Account'])[2]/following::main[1]</value>
      <webElementGuid>19386ecc-72b8-4d12-be25-25989ec84842</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Sign In'])[2]/following::main[1]</value>
      <webElementGuid>3de532eb-bd90-4731-b526-3db65dff3977</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//main</value>
      <webElementGuid>f7be53f2-c8f1-4ad1-b636-808a4e8dd78c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//main[@id = 'maincontent' and (text() = concat(&quot;

    
        Create New Customer Account    
    Personalized recommendations


    

    



    
        window.authenticationPopup = {&quot;autocomplete&quot;:&quot;off&quot;,&quot;customerRegisterUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/create\/&quot;,&quot;customerForgotPasswordUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/forgotpassword\/&quot;,&quot;baseUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/&quot;};
    
    



    







        
        Personal Information
        
        
        
            
            First Name
            
                
            
        
            
            Last Name
            
                
            
        
    
            
                
                
                    
        
        Sign-in Information
        
            Email
            
                
            
        
        
            Password
            
                Minimum of different classes of characters in password is 3. Classes of characters: Lower Case, Upper Case, Digits, Special Characters.
                
                    
                        Password Strength:
                        Very Strong
                    
                
            

        
        
            Confirm Password
            
                Please enter the same value again.
            
        
            
    
        
            Create an Account
        
        
            Back
        
    


require([
    &quot; , &quot;'&quot; , &quot;jquery&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;mage/mage&quot; , &quot;'&quot; , &quot;
], function($){

    var dataForm = $(&quot; , &quot;'&quot; , &quot;#form-validate&quot; , &quot;'&quot; , &quot;);
    var ignore = null;

    dataForm.mage(&quot; , &quot;'&quot; , &quot;validation&quot; , &quot;'&quot; , &quot;, {
            ignore: ignore ? &quot; , &quot;'&quot; , &quot;:hidden:not(&quot; , &quot;'&quot; , &quot; + ignore + &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;
        }).find(&quot; , &quot;'&quot; , &quot;input:text&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;off&quot; , &quot;'&quot; , &quot;);
});



&quot;) or . = concat(&quot;

    
        Create New Customer Account    
    Personalized recommendations


    

    



    
        window.authenticationPopup = {&quot;autocomplete&quot;:&quot;off&quot;,&quot;customerRegisterUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/create\/&quot;,&quot;customerForgotPasswordUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/customer\/account\/forgotpassword\/&quot;,&quot;baseUrl&quot;:&quot;https:\/\/magento.softwaretestingboard.com\/&quot;};
    
    



    







        
        Personal Information
        
        
        
            
            First Name
            
                
            
        
            
            Last Name
            
                
            
        
    
            
                
                
                    
        
        Sign-in Information
        
            Email
            
                
            
        
        
            Password
            
                Minimum of different classes of characters in password is 3. Classes of characters: Lower Case, Upper Case, Digits, Special Characters.
                
                    
                        Password Strength:
                        Very Strong
                    
                
            

        
        
            Confirm Password
            
                Please enter the same value again.
            
        
            
    
        
            Create an Account
        
        
            Back
        
    


require([
    &quot; , &quot;'&quot; , &quot;jquery&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;mage/mage&quot; , &quot;'&quot; , &quot;
], function($){

    var dataForm = $(&quot; , &quot;'&quot; , &quot;#form-validate&quot; , &quot;'&quot; , &quot;);
    var ignore = null;

    dataForm.mage(&quot; , &quot;'&quot; , &quot;validation&quot; , &quot;'&quot; , &quot;, {
            ignore: ignore ? &quot; , &quot;'&quot; , &quot;:hidden:not(&quot; , &quot;'&quot; , &quot; + ignore + &quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;:hidden&quot; , &quot;'&quot; , &quot;
        }).find(&quot; , &quot;'&quot; , &quot;input:text&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;off&quot; , &quot;'&quot; , &quot;);
});



&quot;))]</value>
      <webElementGuid>69d887b7-8a1f-4537-a834-323ef84eca87</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
